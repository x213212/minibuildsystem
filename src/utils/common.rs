use std::collections::HashMap;
use std::process::Command;
use std::sync::Mutex;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs;

// Lazy static variables to store incoming parameters
lazy_static! {
    static ref GLOBAL_VARS: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    static ref SCRIPT_CONFIG: Mutex<HashMap<String, ScriptDetails>> = Mutex::new(HashMap::new());
}

// Structure to match YAML format
#[derive(Debug, Deserialize)]
pub struct ScriptConfig {
    pub scripts: HashMap<String, ScriptDetails>,
}

// Load YAML configuration
pub fn load_script_config(file_path: &str) -> Result<ScriptConfig, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(file_path)?;
    let config: ScriptConfig = serde_yaml::from_str(&contents)?;
    Ok(config)
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScriptDetails {
    pub repo: String,
    pub branch: Option<String>,
}

// Set script configuration in global state
pub fn set_script_config(config: HashMap<String, ScriptDetails>) {
    let mut script_config = SCRIPT_CONFIG.lock().unwrap();
    *script_config = config;
}

// Retrieve script configuration by name
pub fn get_script_config(script_name: &str) -> Option<ScriptDetails> {
    let script_config = SCRIPT_CONFIG.lock().unwrap();
    println!("Available scripts: {:?}", script_config.keys().collect::<Vec<_>>()); // Print all keys
    
    script_config.get(script_name).cloned()
}

// Store parameters in global variables
pub fn set_var(key: &str, value: &str) {
    let mut vars = GLOBAL_VARS.lock().unwrap();
    vars.insert(key.to_string(), value.to_string());
}

// Query parameters from global variables
pub fn get_var(key: &str) -> Option<String> {
    let vars = GLOBAL_VARS.lock().unwrap();
    vars.get(key).cloned()
}

// Execute a command with the specified environment variables
pub fn execute_with_env(command: &str, env_vars: HashMap<String, String>) -> Result<(), String> {
    let mut cmd = Command::new("bash");
    cmd.arg("-c").arg(command);

    // Set environment variables
    for (key, value) in &env_vars {
        cmd.env(key, value);
    }

    // Execute command and capture output
    let output = cmd.output().map_err(|e| format!("Command execution failed: {}", e))?;

    if output.status.success() {
        println!("Command executed successfully: {}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    } else {
        Err(format!(
            "Command execution failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
