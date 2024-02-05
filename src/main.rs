use clap::Parser;
use serde_derive::Deserialize;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use toml;

#[derive(Parser, Debug)]
struct Args {
    /// The name of the plan to execute
    plan_name: String,
}

#[derive(Deserialize, Debug)]
struct PlanConfig {
    meta: PlanMeta,
    inputs: PlanInputs,
    files: Vec<PlanFile>,
}

#[derive(Deserialize, Debug)]
struct PlanMeta {
    name: String,
    version: String,
}

#[derive(Deserialize, Debug)]
struct PlanInputs {
    inputs: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct PlanFile {
    template_file: String,
    output_file: String,
}

#[derive(Debug)]
struct KeyVal {
    key: String,
    val: String,
}

fn main() {
    let args = Args::parse();

    let plan: PlanConfig = read_plan(args.plan_name);

    println!(
        "Executing plan: {:?} - Version: {:?}\n---\n",
        plan.meta.name, plan.meta.version
    );

    println!("Please provide the following inputs:\n");

    let mut input_list = Vec::<KeyVal>::new();
    plan.inputs.inputs.iter().for_each(|input_key| {
        println!("Input value for key: {:?}", input_key);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {}
            Err(_no_updates_is_fine) => {}
        }
        let input_val = input.trim().to_string();
        input_list.push(KeyVal {
            key: input_key.to_string(),
            val: input_val,
        });
    });

    println!("\nCreating files...\n");

    plan.files.iter().for_each(|file| {
        transpile_and_save_template(&plan.meta.name, file, &input_list);
    });

    println!("\nFinished executing plan {:?}!", plan.meta.name);
}

fn transpile_and_save_template(plan_name: &String, template: &PlanFile, input_list: &Vec<KeyVal>) {
    let mut template_dir = env::current_exe().unwrap();
    template_dir.pop();
    template_dir.push("plans");
    template_dir.push(plan_name);

    let template_file_path = template_dir.join(template.template_file.to_string());

    let template_file_content = match fs::read_to_string(&template_file_path) {
        Ok(contents) => contents,
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file: `{}`", template_file_path.display());
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    // println!("Template: {:?}", template_file_content);

    // Replace all keys with their values in the file's content.
    let mut transpiled_file_content = template_file_content.to_string();
    for kv in input_list {
        let key = format!("{{{{{}}}}}", kv.key);
        let val = kv.val.to_string();
        transpiled_file_content = transpiled_file_content.replace(&key, &val);
    }

    // println!("Transpiled: {:?}", transpiled_file_content);

    // Create output file path by replacing all keys with their values.
    let mut output_file_path = template.output_file.to_string();
    for kv in input_list {
        let key = format!("{{{{{}}}}}", kv.key);
        let val = kv.val.to_string();
        output_file_path = output_file_path.replace(&key, &val);
    }

    // println!("Output file path: {:?}", output_file_path);

    let output_path_buf = PathBuf::from(output_file_path);

    // Create the output folders, if they don't exist.
    if let Some(parent_dir) = output_path_buf.parent() {
        if let Err(error) = fs::create_dir_all(parent_dir) {
            eprintln!(
                "Failed to create folders for path '{}' : {}",
                parent_dir.display(),
                error
            );
            // Exit the program with exit code `1`.
            exit(1);
        }
    }

    // Now create and write to the file.
    let mut file = match fs::File::create(&output_path_buf) {
        Ok(file) => file,
        Err(err) => {
            eprintln!(
                "Failed to create file at '{}': {}",
                output_path_buf.display(),
                err
            );
            exit(1);
        }
    };

    if let Err(e) = file.write_all(transpiled_file_content.as_bytes()) {
        eprintln!(
            "Failed to write to file at '{}': {}",
            output_path_buf.display(),
            e
        );
        exit(1);
    }

    println!("Created file: {:?}", output_path_buf);
}

fn read_plan(plan_name: String) -> PlanConfig {
    let mut plan_file_path = env::current_exe().unwrap();
    plan_file_path.pop();
    plan_file_path.push("plans");
    plan_file_path.push(format!("{}.plan.toml", plan_name));

    println!("Plan file path: {:?}", plan_file_path.display());

    let plan_file_path_str = plan_file_path.to_str().unwrap();

    let contents = match fs::read_to_string(plan_file_path_str) {
        Ok(contents) => contents,
        Err(_) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not read file `{}`", plan_file_path_str);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    // println!("Contents: {:?}", contents);

    let plan: PlanConfig = match toml::from_str(&contents) {
        Ok(plan) => plan,
        Err(err) => {
            // Write `msg` to `stderr`.
            eprintln!("Could not parse TOML file `{}`.", plan_file_path_str);
            eprintln!("Error: {:?}", err);
            // Exit the program with exit code `1`.
            exit(1);
        }
    };

    return plan;
}
