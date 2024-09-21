pub mod test;   // Change module imports to submodules
pub mod test2;  // Change module imports to submodules
pub mod test3;  // Change module imports to submodules

use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::sync::{Mutex, LazyLock}; // Import necessary synchronization primitives

type ScriptFunc = fn(&HashMap<String, String>) -> Result<(), String>;
type DependencyFunc = fn() -> Vec<(String, HashMap<String, String>)>;

#[derive(Default)]
struct BuildStatus {
    version: Option<String>,
    additional_params: HashMap<String, String>,
    source_dir: Option<String>, // To hold the path of the source package
}

// Global variable to hold build statuses for different scripts
static BUILD_STATUSES: LazyLock<Mutex<HashMap<String, BuildStatus>>> = LazyLock::new(|| {
    Mutex::new(HashMap::new())
});

// Retrieve all scripts
pub fn get_scripts() -> HashMap<String, ScriptFunc> {
    let mut scripts: HashMap<String, ScriptFunc> = HashMap::new();
    
    // Update to correct paths
    scripts.insert("test".to_string(), test::run_test);
    scripts.insert("test2".to_string(), test2::run_test);
    scripts.insert("test3".to_string(), test3::run_test);
    
    scripts
}

// Automatically retrieve dependencies
pub fn get_dependency_functions() -> HashMap<String, DependencyFunc> {
    let mut dependencies: HashMap<String, DependencyFunc> = HashMap::new();

    dependencies.insert("test3".to_string(), test3::get_dependencies);

    dependencies
}

// Define Trait
pub trait CloneRepo {
    fn clone_or_pull(&self, repo_url: &str, branch: Option<&str>, target_dir: &str, should_pull: bool) -> Result<String, String>;
}

// Provide default implementation
pub struct DefaultClone;

impl CloneRepo for DefaultClone {
    fn clone_or_pull(&self, repo_url: &str, branch: Option<&str>, target_dir: &str, should_pull: bool) -> Result<String, String> {
        // Check if the directory already exists
        if Path::new(target_dir).exists() {
            if should_pull {
                // If the directory exists and a pull is required, execute git pull
                println!("Directory already exists: {}", target_dir);
                println!("Pulling latest changes in directory: {}", target_dir);
                
                let output = Command::new("git")
                    .args(&["pull"])
                    .current_dir(target_dir)
                    .status()
                    .map_err(|e| e.to_string())?;

                if !output.success() {
                    return Err("Failed to pull the repository".to_string());
                }
            } else {
                println!("Skipping pull; using existing directory: {}", target_dir);
            }
        } else {
            // Execute git clone
            println!("Cloning repo: {}{}", repo_url, branch.map_or(String::new(), |b| format!(" (branch: {})", b)));
            
            let output = Command::new("git")
                .args(&["clone", repo_url, target_dir])
                .status()
                .map_err(|e| e.to_string())?;

            if !output.success() {
                return Err("Failed to clone repository".to_string());
            }

            // // If a branch is specified, switch to that branch
            // if let Some(branch_name) = branch {
            //     let output = Command::new("git")
            //         .args(&["checkout", branch_name])
            //         .current_dir(target_dir)
            //         .status()
            //         .map_err(|e| e.to_string())?;

            //     if !output.success() {
            //         return Err(format!("Failed to checkout branch '{}'", branch_name));
            //     }
            // }
        }

        Ok(target_dir.to_string()) // Return the path of the cloned directory
    }
}
