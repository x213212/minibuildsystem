use std::collections::HashMap;
use crate::buildscript::{CloneRepo, DefaultClone}; // Import CloneRepo trait and its default implementation
use crate::utils::common::{get_script_config, get_var, execute_with_env}; // Import necessary functions
use std::sync::Mutex; // Import Mutex for thread safety
use std::sync::LazyLock; // Import LazyLock for lazy initialization

// Assuming BUILD_STATUSES is declared somewhere globally
use crate::buildscript::BUILD_STATUSES;

pub fn run_test(extra_params: &HashMap<String, String>) -> Result<(), String> {
    let script_name = "test"; // Based on the currently executed script name

    // Lock the BUILD_STATUSES Mutex
    let mut statuses = BUILD_STATUSES.lock().unwrap(); // Locking the Mutex to safely access the HashMap
    let status = statuses.entry(script_name.to_string()).or_default(); // Initialize or get the existing BuildStatus

    println!("Executing build script for the test tool");

    // Print additional parameters
    for (key, value) in extra_params {
        println!("Additional parameter: {} = {}", key, value);
        status.additional_params.insert(key.clone(), value.clone()); // Store additional params
    }

    // Retrieve version parameter from global variables
    let version = get_var("version").unwrap_or_else(|| "Not provided".to_string());
    println!("Using version: {}", version);
    status.version = Some(version.clone()); // Store version in BuildStatus

    // Retrieve Git repository information for the script
    let details = get_script_config(script_name).ok_or("Script configuration not found")?;
    println!("Git repository: {}", details.repo);

    let default_branch = "master".to_string();
    let branch_name = details.branch.as_ref().unwrap_or(&default_branch);
    println!("Branch: {}", branch_name);

    // Construct the directory name
    let target_dir = format!("sourcepackage/{}_{}_{}", script_name, version, branch_name);
    status.source_dir = Some(target_dir.clone()); // Store source_dir in BuildStatus

    // Use the default cloning logic
    let cloner = DefaultClone;
    let should_pull = true;
    cloner.clone_or_pull(&details.repo, Some(branch_name), &target_dir, should_pull)?;

    // Create custom environment variables
    let mut myenv = HashMap::new();
    myenv.insert("a".to_string(), "10".to_string());

    match execute_with_env("echo $a", myenv) {
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
