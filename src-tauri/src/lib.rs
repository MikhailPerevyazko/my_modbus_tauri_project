use serde::{Deserialize, Serialize};
use std::fs;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            load_connections_config,
            load_param_configs,
            json_connection_config,
            json_param_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Connection {
    pub host: Option<String>,
    pub port: Option<String>,
    pub listen_host: Option<String>,
    pub listen_port: Option<String>,
    pub name: Option<String>,
    pub baud_rate: Option<i32>,
    pub data_bits: Option<i32>,
    pub flow_control: Option<String>,
    pub parity: Option<String>,
    pub stop_bits: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectionModbusConfig {
    pub protocol_type: String,
    pub connection: Connection,
    pub channel_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionParameter {
    pub name: String,
    pub unit_id: i32,
    pub ptype: String,
    pub start_address: i32,
    pub mstorage: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientModbusConfigs {
    pub connection_configs: Vec<ConnectionModbusConfig>,
    pub parameters: Vec<FunctionParameter>,
}

#[tauri::command]
fn load_connections_config(path_to_config: &str) -> Vec<ConnectionModbusConfig> {
    let configuration = fs::read_to_string(path_to_config.to_string()).unwrap();
    let configs: ClientModbusConfigs = serde_yaml::from_str(&configuration.as_str()).unwrap();
    let vec_client_modbus_config = configs.connection_configs;
    vec_client_modbus_config
}

#[tauri::command]
fn load_param_configs(path_to_config: &str) -> Vec<FunctionParameter> {
    let configuration = fs::read_to_string(path_to_config.to_string()).unwrap();
    let configs: ClientModbusConfigs = serde_yaml::from_str(&configuration.as_str()).unwrap();
    let parameters = configs.parameters;
    parameters
}

#[tauri::command]
fn json_connection_config() -> Vec<ConnectionModbusConfig> {
    let path = String::from("/home/Mikhail/projects/modbus_tcp/modbus_config.yaml");
    let configs = load_connections_config(&path);
    configs
}

#[tauri::command]
fn json_param_config() -> Vec<FunctionParameter> {
    let path = String::from("/home/Mikhail/projects/modbus_tcp/modbus_config.yaml");
    let configs = load_param_configs(&path);
    configs
}
