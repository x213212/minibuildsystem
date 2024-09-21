use std::collections::HashMap;
use crate::utils::common::{execute_with_env, get_var};

pub fn run_test(extra_params: &HashMap<String, String>) -> Result<(), String> {
    println!("Executing build script for test2");

    // Print additional parameters
    for (key, value) in extra_params {
        println!("Additional parameter: {} = {}", key, value);
    }

    // Retrieve version parameter from global variables
    if let Some(version) = get_var("version") {
        println!("Using version: {}", version);
    } else {
        println!("Version parameter not provided");
    }

    // Create custom environment variables
    let mut myenv = HashMap::new();
    myenv.insert("b".to_string(), "20".to_string());

    match execute_with_env("echo $b", myenv) {
        Ok(_) => {
            println!("Build successful");
            Ok(())
        },
        Err(e) => {
            eprintln!("Build failed: {}", e);
            Err(e.to_string())
        },
    }
}
