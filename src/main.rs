use std::env;
use std::io::{self, Write};
use std::collections::HashMap;
mod buildscript;
mod utils;

fn main() {
    // Load configuration
    let config = match utils::common::load_script_config("/app/build_scripts_gitrepo.yml") {
        Ok(config) => config,
        Err(_) => {
            // Fallback to current directory if not found in /app
            utils::common::load_script_config("build_scripts_gitrepo.yml")
                .expect("Failed to load config")
        }
    };

    // Ensure config.scripts returns a type of HashMap<String, utils::common::ScriptDetails>
    let mut script_config = HashMap::new();
    for (name, details) in config.scripts {
        // Ensure that details are of type utils::common::ScriptDetails
        script_config.insert(name, details);
    }

    // Store the loaded script configuration in global variables
    utils::common::set_script_config(script_config);

    // Retrieve command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let package_name = &args[1];

        // Process arguments in the format key==value
        for arg in &args[2..] {
            if let Some((key, value)) = parse_arg(arg) {
                utils::common::set_var(&key, &value);  // Store parameters in global variables
            }
        }

        // Create a HashMap for additional parameters
        let extra_params = HashMap::new(); // Populate these parameters as needed

        // Dynamically execute the corresponding script
        match execute_script_with_params(package_name, extra_params) {
            Ok(_) => return,
            Err(e) => eprintln!("Execution failed: {}", e),
        }
    } else {
        show_menu();
    }
}

// Parse parameters, returning (key, value) if in key==value format
fn parse_arg(arg: &str) -> Option<(String, String)> {
    if let Some(pos) = arg.find("==") {
        let key = arg[..pos].to_string();
        let value = arg[pos + 2..].to_string();
        Some((key, value))
    } else {
        None
    }
}

fn execute_script_with_params(script_name: &str, params: HashMap<String, String>) -> Result<(), String> {
    let scripts = buildscript::get_scripts();
    let dependency_functions = buildscript::get_dependency_functions();

    // Retrieve script dependencies and their parameters
    if let Some(get_deps) = dependency_functions.get(script_name) {
        let dependencies = get_deps(); // Call the dependency function

        for (dep_name, dep_params) in dependencies {
            execute_script_with_params(&dep_name, dep_params)?; // Call recursively
        }
    }

    // Execute the main script
    if let Some(script_func) = scripts.get(script_name) {
        println!("Executing {}.rs...", script_name);
        script_func(&params)?; // Pass parameters to the script function
    } else {
        return Err(format!("Unknown script: {}", script_name));
    }

    println!("{} executed successfully", script_name); // Print success message only once
    println!("=========================");
    Ok(()) // Return Ok at the end
}


// Display the menu and execute the selected script
fn show_menu() {
    let scripts = buildscript::get_scripts(); // Retrieve registered scripts
    if scripts.is_empty() {
        println!("No executable scripts available.");
        return;
    }

    // Sort scripts alphabetically
    let mut sorted_scripts: Vec<_> = scripts.keys().collect();
    sorted_scripts.sort();

    println!("Please select the script to execute:");
    for (index, script) in sorted_scripts.iter().enumerate() {
        println!("{}. {}", index + 1, script);
    }

    let mut input = String::new();
    print!("Please enter the script number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Unable to read input");
    let choice: usize = input.trim().parse().unwrap_or(0);

    if choice == 0 || choice > sorted_scripts.len() {
        println!("Invalid selection.");
        return;
    }

    let script_name = sorted_scripts[choice - 1];
    println!("Executing script: {}", script_name);

    match execute_script_with_params(script_name, HashMap::new()) { // Pass empty parameters
        Ok(_) => (),
        Err(e) => println!("Execution failed: {}", e),
    }
}
