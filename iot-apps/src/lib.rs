//! SecureIoTOS 
//! License: Apache 2.0
//! Author: Md Mahbubur Rahman
//! URL: https://m-a-h-b-u-b.github.io
//! GitHub: https://github.com/m-a-h-b-u-b/SecureIoTOS

pub mod hello;
pub mod sensor;
pub mod telemetry;

pub async fn run_demo() {

    hello::hello_world();
    
    let temp = sensor::read_sensor();
    sensor::send_sensor_data(temp);
    
    let data = telemetry::collect_telemetry();
    telemetry::transmit_telemetry(&data);
}