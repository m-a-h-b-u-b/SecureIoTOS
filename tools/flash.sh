#!/usr/bin/env bash
#
# SecureIoTOS – Flash Utility
#
# Provides a universal flashing helper for SecureIoTOS firmware images.
# Supports probe-rs, OpenOCD + st-flash, esptool.py, and dfu-util.
#
# License: Apache 2.0
# Author: Md Mahbubur Rahman
# Project URL: https://m-a-h-b-u-b.github.io
# GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS
#

set -euo pipefail
IFS=$'\n\t'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="${SCRIPT_DIR%/tools}"
DRY_RUN=0
BOARD=""
PORT=""
TARGET=""
IMAGE=""
METHOD="auto"
NO_BUILD=0
TOOL=""
CHIP=""

log(){ echo -e "[flash] $*" >&2; }
err(){ echo -e "[flash][ERROR] $*" >&2; exit 1; }
run(){ if [ "$DRY_RUN" -eq 1 ]; then echo "DRY RUN: $*"; else echo "+ $*"; eval "$*"; fi }

usage(){ cat <<EOF
Usage: $(basename "$0") [options]
Options:
  -b, --board BOARD        Board id (e.g. stm32f4, nrf52840, esp32)
  -p, --port PORT          Serial port or device (e.g. /dev/ttyUSB0)
  -t, --target TARGET      Cargo target triple
  -i, --image IMAGE        Path to the firmware image (.elf or .bin). If omitted, builds the repo.
  -m, --method METHOD      Flash method: auto, probe, openocd, esptool, dfu
  --tool TOOL              Explicit tool binary
  -n, --no-build           Don't run cargo build; assume IMAGE provided
  --chip CHIP              Chip name for esptool or st-flash
  --dry-run                Show commands that would run and exit
  -h, --help               Show this help
EOF
exit 1 }

# Simple arg parsing
while [[ $# -gt 0 ]]; do
  case "$1" in
    -b|--board) BOARD="$2"; shift 2;;
    -p|--port) PORT="$2"; shift 2;;
    -t|--target) TARGET="$2"; shift 2;;
    -i|--image) IMAGE="$2"; shift 2;;
    -m|--method) METHOD="$2"; shift 2;;
    --tool) TOOL="$2"; shift 2;;
    -n|--no-build) NO_BUILD=1; shift;;
    --chip) CHIP="$2"; shift 2;;
    --dry-run) DRY_RUN=1; shift;;
    -h|--help) usage;;
    --) shift; break;;
    -*) err "Unknown option: $1";;
    *) break;;
  esac
done

# Helpers to detect tools
has_cmd(){ command -v "$1" >/dev/null 2>&1; }

infer_target(){
  if [ -n "$TARGET" ]; then return; fi
  if [ -f "${ROOT_DIR}/Cargo.toml" ]; then
    if grep -q "thumbv7em" "${ROOT_DIR}/Cargo.toml" 2>/dev/null; then TARGET="thumbv7em-none-eabihf"; fi
  fi
}

build_image(){
  if [ "$NO_BUILD" -eq 1 ]; then log "Skipping build (--no-build)"; return; fi
  log "Building firmware (release)..."
  if [ -n "$TARGET" ]; then
    run "cargo build --release --target ${TARGET}"
  else
    run "cargo build --release"
  fi
  if [ -z "$IMAGE" ]; then
    IMAGE_CANDIDATE=$(find target -type f -path "*/release/*.elf" -print0 2>/dev/null | xargs -0 ls -1 -S 2>/dev/null | head -n1 || true)
    if [ -n "$IMAGE_CANDIDATE" ]; then
      IMAGE="$IMAGE_CANDIDATE"
      log "Auto-detected IMAGE=$IMAGE"
    else
      err "Couldn't find built .elf. Provide --image or set TARGET to a valid target." 
    fi
  fi
}

flash_with_probe_rs(){
  if has_cmd probe-rs-cli; then
    log "Flashing with probe-rs-cli"
    run "probe-rs-cli download ${IMAGE} --chip ${CHIP}"
    return 0
  fi
  if has_cmd cargo && has_cmd probe-run; then
    log "Running with probe-run (executes the program via probe)"
    if [ -n "$TARGET" ]; then
      run "cargo run --release --target ${TARGET} --bin $(basename ${IMAGE%.*}) -- --probe-run"
    else
      run "probe-run ${IMAGE}"
    fi
    return 0
  fi
  return 1
}

flash_with_openocd_stlink(){
  if has_cmd openocd && has_cmd st-flash; then
    log "Flashing with openocd + st-flash"
    if [[ "$IMAGE" == *.bin ]]; then
      run "st-flash write ${IMAGE} 0x8000000"
    else
      tmpbin="/tmp/firmware-$$.bin"
      run "arm-none-eabi-objcopy -O binary ${IMAGE} ${tmpbin}"
      run "st-flash write ${tmpbin} 0x8000000"
      run "rm -f ${tmpbin}"
    fi
    return 0
  fi
  if has_cmd openocd; then
    log "Flashing with openocd (attempt)"
    run "openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c 'program ${IMAGE} verify reset; shutdown'"
    return 0
  fi
  return 1
}

flash_with_esptool(){
  if has_cmd esptool.py; then
    if [ -z "$PORT" ]; then err "esptool requires --port"; fi
    log "Flashing ESP with esptool.py"
    if [[ "$IMAGE" == *.elf ]]; then
      tmpbin="/tmp/esp-$$.bin"
      run "xtensa-esp32-elf-objcopy -O binary ${IMAGE} ${tmpbin} || arm-none-eabi-objcopy -O binary ${IMAGE} ${tmpbin} || (echo 'objcopy failed' && false)"
      IMAGE_TO_FLASH=${tmpbin}
    else
      IMAGE_TO_FLASH=${IMAGE}
    fi
    if [ -n "$CHIP" ]; then
      run "esptool.py --chip ${CHIP} --port ${PORT} write_flash -z 0x1000 ${IMAGE_TO_FLASH}"
    else
      run "esptool.py --port ${PORT} write_flash -z 0x1000 ${IMAGE_TO_FLASH}"
    fi
    [ -n "${tmpbin-}" ] && run "rm -f ${tmpbin}"
    return 0
  fi
  return 1
}

flash_with_dfu(){
  if has_cmd dfu-util; then
    if [ -z "$PORT" ]; then log "dfu-util usually autodetects USB DFU device"; fi
    log "Flashing with dfu-util"
    if [[ "$IMAGE" == *.elf ]]; then
      tmpbin="/tmp/dfu-$$.bin"
      run "objcopy -O binary ${IMAGE} ${tmpbin}"
      IMAGE_TO_FLASH=${tmpbin}
    else
      IMAGE_TO_FLASH=${IMAGE}
    fi
    run "dfu-util -a 0 -D ${IMAGE_TO_FLASH}"
    [ -n "${tmpbin-}" ] && run "rm -f ${tmpbin}"
    return 0
  fi
  return 1
}

main(){
  infer_target
  if [ -z "$IMAGE" ] && [ "$NO_BUILD" -eq 0 ]; then
    build_image
  fi
  if [ -z "$IMAGE" ]; then
    err "No image to flash. Provide --image or ensure build succeeds."
  fi

  log "Flashing image: ${IMAGE}"

  if [ "$METHOD" != "auto" ]; then
    case "$METHOD" in
      probe) flash_with_probe_rs || err "probe method failed";;
      openocd) flash_with_openocd_stlink || err "openocd method failed";;
      esptool) flash_with_esptool || err "esptool method failed";;
      dfu) flash_with_dfu || err "dfu method failed";;
      *) err "Unknown method: ${METHOD}";;
    esac
    log "Done."
    exit 0
  fi

  if flash_with_probe_rs; then log "Flashed using probe-rs"; exit 0; fi
  if flash_with_openocd_stlink; then log "Flashed using openocd/st-flash"; exit 0; fi
  if flash_with_esptool; then log "Flashed using esptool"; exit 0; fi
  if flash_with_dfu; then log "Flashed using dfu-util"; exit 0; fi

  err "No supported flashing tool found or flashing failed. Install probe-rs-cli (recommended) or openocd/st-flash/esptool/dfu-util."
}

main "$@"
