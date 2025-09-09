#!/usr/bin/env bash
#
# SecureIoTOS – Flash Utility
#
# Universal flashing helper for SecureIoTOS firmware images.
# Supports probe-rs, OpenOCD + st-flash, esptool.py, and dfu-util.
#
# License : Dual License
#           - Apache 2.0 for open-source / personal use
#           - Commercial license required for closed-source use
# Author  : Md Mahbubur Rahman
# Project : https://m-a-h-b-u-b.github.io
# GitHub  : https://github.com/m-a-h-b-u-b/SecureIoTOS
#

set -euo pipefail
IFS=$'\n\t'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="${SCRIPT_DIR%/tools}"

# -------------------------
# Global state
# -------------------------
DRY_RUN=0
BOARD=""
PORT=""
TARGET=""
IMAGE=""
METHOD="auto"
NO_BUILD=0
TOOL=""
CHIP=""

# -------------------------
# Logging helpers
# -------------------------
log() { echo -e "[flash] $*" >&2; }
warn(){ echo -e "[flash][WARN] $*" >&2; }
err() { echo -e "[flash][ERROR] $*" >&2; exit 1; }

# -------------------------
# Run wrapper (dry-run safe)
# -------------------------
run() {
  if [ "$DRY_RUN" -eq 1 ]; then
    echo "DRY RUN: $*"
  else
    echo "+ $*"
    eval "$*"
  fi
}

# -------------------------
# Usage
# -------------------------
usage() {
  cat <<EOF
Usage: $(basename "$0") [options]

Options:
  -b, --board BOARD     Board ID (e.g., stm32f4, nrf52840, esp32)
  -p, --port PORT       Serial port/device (e.g., /dev/ttyUSB0)
  -t, --target TARGET   Cargo target triple
  -i, --image IMAGE     Firmware image (.elf or .bin). If omitted, builds.
  -m, --method METHOD   Flash method: auto, probe, openocd, esptool, dfu
  --tool TOOL           Explicit tool binary path
  -n, --no-build        Skip cargo build (requires --image)
  --chip CHIP           Chip name for esptool/st-flash
  --dry-run             Print commands instead of executing
  -h, --help            Show this help
EOF
  exit 1
}

# -------------------------
# Argument parsing
# -------------------------
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

# -------------------------
# Utility helpers
# -------------------------
has_cmd(){ command -v "$1" >/dev/null 2>&1; }

infer_target() {
  if [ -n "$TARGET" ]; then return; fi
  if [ -f "${ROOT_DIR}/Cargo.toml" ]; then
    if grep -q "thumbv7em" "${ROOT_DIR}/Cargo.toml"; then
      TARGET="thumbv7em-none-eabihf"
    fi
  fi
}

# -------------------------
# Build firmware
# -------------------------
build_image() {
  if [ "$NO_BUILD" -eq 1 ]; then
    log "Skipping build (--no-build)"
    return
  fi

  log "Building firmware (release)..."
  if [ -n "$TARGET" ]; then
    run "cargo build --release --target ${TARGET}"
  else
    run "cargo build --release"
  fi

  if [ -z "$IMAGE" ]; then
    IMAGE="$(find target -type f -path "*/release/*.elf" -print0 2>/dev/null | \
              xargs -0 ls -1 -S 2>/dev/null | head -n1 || true)"
    [ -n "$IMAGE" ] || err "No .elf found. Provide --image or a valid --target."
    log "Auto-detected IMAGE=$IMAGE"
  fi
}

# -------------------------
# Flash methods
# -------------------------
flash_with_probe_rs() {
  if has_cmd probe-rs-cli; then
    [ -n "$CHIP" ] || err "probe-rs requires --chip"
    log "Flashing with probe-rs-cli"
    run "probe-rs-cli download ${IMAGE} --chip ${CHIP}"
    return 0
  fi

  if has_cmd cargo && has_cmd probe-run; then
    log "Running with probe-run"
    if [ -n "$TARGET" ]; then
      run "cargo run --release --target ${TARGET} --bin $(basename ${IMAGE%.*}) -- --probe-run"
    else
      run "probe-run ${IMAGE}"
    fi
    return 0
  fi
  return 1
}

flash_with_openocd_stlink() {
  if has_cmd st-flash; then
    log "Flashing with st-flash"
    if [[ "$IMAGE" == *.bin ]]; then
      run "st-flash write ${IMAGE} 0x8000000"
    else
      tmpbin="$(mktemp /tmp/fw-XXXXXX.bin)"
      run "arm-none-eabi-objcopy -O binary ${IMAGE} ${tmpbin}"
      run "st-flash write ${tmpbin} 0x8000000"
      rm -f "$tmpbin"
    fi
    return 0
  fi

  if has_cmd openocd; then
    log "Flashing with openocd"
    run "openocd -f interface/stlink.cfg -f target/stm32f4x.cfg -c 'program ${IMAGE} verify reset; shutdown'"
    return 0
  fi
  return 1
}

flash_with_esptool() {
  has_cmd esptool.py || return 1
  [ -n "$PORT" ] || err "esptool requires --port"

  log "Flashing ESP with esptool.py"
  if [[ "$IMAGE" == *.elf ]]; then
    tmpbin="$(mktemp /tmp/esp-XXXXXX.bin)"
    run "xtensa-esp32-elf-objcopy -O binary ${IMAGE} ${tmpbin} || arm-none-eabi-objcopy -O binary ${IMAGE} ${tmpbin}"
    IMAGE_TO_FLASH="$tmpbin"
  else
    IMAGE_TO_FLASH="$IMAGE"
  fi

  if [ -n "$CHIP" ]; then
    run "esptool.py --chip ${CHIP} --port ${PORT} write_flash -z 0x1000 ${IMAGE_TO_FLASH}"
  else
    run "esptool.py --port ${PORT} write_flash -z 0x1000 ${IMAGE_TO_FLASH}"
  fi

  [ -n "${tmpbin-}" ] && rm -f "$tmpbin"
  return 0
}

flash_with_dfu() {
  has_cmd dfu-util || return 1

  log "Flashing with dfu-util"
  if [[ "$IMAGE" == *.elf ]]; then
    tmpbin="$(mktemp /tmp/dfu-XXXXXX.bin)"
    run "objcopy -O binary ${IMAGE} ${tmpbin}"
    IMAGE_TO_FLASH="$tmpbin"
  else
    IMAGE_TO_FLASH="$IMAGE"
  fi

  run "dfu-util -a 0 -D ${IMAGE_TO_FLASH}"
  [ -n "${tmpbin-}" ] && rm -f "$tmpbin"
  return 0
}

# -------------------------
# Main
# -------------------------
main() {
  infer_target
  if [ -z "$IMAGE" ] && [ "$NO_BUILD" -eq 0 ]; then
    build_image
  fi
  [ -n "$IMAGE" ] || err "No image to flash. Provide --image or enable build."

  log "Flashing image: ${IMAGE}"

  if [ "$METHOD" != "auto" ]; then
    case "$METHOD" in
      probe)   flash_with_probe_rs   || err "probe method failed";;
      openocd) flash_with_openocd_stlink || err "openocd method failed";;
      esptool) flash_with_esptool    || err "esptool method failed";;
      dfu)     flash_with_dfu        || err "dfu method failed";;
      *) err "Unknown method: ${METHOD}";;
    esac
    log "Done."
    exit 0
  fi

  flash_with_probe_rs        && { log "Flashed using probe-rs"; exit 0; }
  flash_with_openocd_stlink  && { log "Flashed using openocd/st-flash"; exit 0; }
  flash_with_esptool         && { log "Flashed using esptool"; exit 0; }
  flash_with_dfu             && { log "Flashed using dfu-util"; exit 0; }

  err "No supported flashing tool found. Install probe-rs-cli (recommended), st-flash, openocd, esptool, or dfu-util."
}

main "$@"
