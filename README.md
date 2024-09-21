# Build Manager

## Introduction
Build Manager is a tool for managing and executing build processes, designed to support dynamic script loading and custom environment variables. Its design principles emphasize modularity, flexibility, and extensibility, aiming to simplify the development process and enhance productivity.

## Features
- **Multi-Script Support**: Capable of executing multiple script modules (such as `test`, `test2`, `test3`).
- **Dependency Management**: Automatically identifies and handles dependencies between scripts, ensuring the correct execution order.
- **Flexible Parameter Setup**: Manages script parameters through YAML configuration files, supporting additional custom parameters.
- **Environment Variable Usage**: Supports the creation and use of custom environment variables to flexibly configure script execution environments.
- **Docker Support**: Establishes a unified environment for all build scripts by running in a Docker environment, simplifying deployment processes and ensuring consistency.
nt, simplifying deployment processes and ensuring consistency.


## Usage

### Direct Execution
Use the following command to run a specified script:
```bash
cargo run <script_name> [args...]
```

Example:

```bash
cargo run test3 version==rko-100
```

Using Docker
If you wish to run in a Docker environment, use the following command:
```bash
./docker-run.sh <image_tag> <script_name> [args...]
```

Example:
```bash

./docker-run.sh rsbuild_image:20240921171537 test3 version==rko-100
```
Build Docker Image
To build the Docker image, use the following command:

```bash
./docker-image-build.sh
```
This will generate a Docker image tagged with the current timestamp.

Adding Test Scripts
Steps
Create a New Module Folder: In the src/buildscript directory, create a new module folder (e.g., test4).
Create mod.rs: In the new module folder, create a mod.rs file and implement the run_test function.
Import the New Module: In src/buildscript/mod.rs, import the new module.
Example test4/mod.rs:

```rust

use std::collections::HashMap;

pub fn run_test(extra_params: &HashMap<String, String>) -> Result<(), String> {
    println!("Executing build script for test4");

    for (key, value) in extra_params {
        println!("Additional parameter: {} = {}", key, value);
    }

    Ok(())
}
```
Dependency Example
In test4, to retrieve the status of dependencies, you can create a function to check and display the build status of dependencies:

Example test4/mod.rs:

```rust
use std::collections::HashMap;
use crate::buildscript::BUILD_STATUSES; // Import global BUILD_STATUSES

pub fn run_test(extra_params: &HashMap<String, String>) -> Result<(), String> {
    println!("Executing build script for test4");

    // Print additional parameters
    for (key, value) in extra_params {
        println!("Additional parameter: {} = {}", key, value);
    }

    // Retrieve dependency statuses
    let dependencies = get_dependencies(); // Assume there is a function to get dependencies
    for (dep_name, dep_params) in dependencies {
        println!("Checking status for dependency: {}", dep_name);

        // Safely access build status
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

    Ok(())
}

// Example function to get dependencies
pub fn get_dependencies() -> Vec<(String, HashMap<String, String>)> {
    let mut dependencies = Vec::new();
    dependencies.push(("test".to_string(), HashMap::new()));
    dependencies.push(("test2".to_string(), HashMap::new()));
    dependencies
}
```
