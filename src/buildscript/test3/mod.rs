use std::collections::HashMap;
use crate::utils::common::get_var; // Import necessary functions
use crate::buildscript::BUILD_STATUSES; // Import the global BUILD_STATUSES

pub fn run_test(extra_params: &HashMap<String, String>) -> Result<(), String> {
    println!("Executing build script for test3");

    // Retrieve version parameter from global variables
    let version = get_var("version").unwrap_or_else(|| {
        println!("Version parameter not provided");
        "Not provided".to_string()
    });
    println!("Using version: {}", version);

    // Print additional parameters
    for (key, value) in extra_params {
        println!("Additional parameter: {} = {}", key, value);
    }

    // Retrieve dependencies and print their statuses
    let dependencies = get_dependencies();
    for (dep_name, dep_params) in dependencies {
        println!("Dependency: {}", dep_name);

        // Print the parameters for each dependency
        for (key, value) in dep_params {
            println!("  {} = {}", key, value);
        }

        // Safely access the build status for each dependency
        let status_guard = BUILD_STATUSES.lock().unwrap();
        if let Some(status) = status_guard.get(&dep_name) {
            if let Some(dep_version) = &status.version {
                println!("  Dependency version: {}", dep_version);
            }
            if let Some(source_dir) = &status.source_dir {
                println!("  Dependency source directory: {}", source_dir);
            }
            for (key, value) in &status.additional_params {
                println!("  {} = {}", key, value);
            }
        } else {
            println!("  No status found for dependency: {}", dep_name);
        }
    }

    // Other logic...
    Ok(())
}

// Retrieve dependencies for test3 along with their parameters
pub fn get_dependencies() -> Vec<(String, HashMap<String, String>)> {
    let mut dependencies = Vec::new();

    dependencies.push(("test".to_string(), {
        let mut params = HashMap::new();
        params.insert("param1".to_string(), "value1".to_string());
        params
    }));

    dependencies.push(("test2".to_string(), {
        let mut params = HashMap::new();
        params.insert("paramA".to_string(), "valueA".to_string());
        params
    }));

    dependencies
}
