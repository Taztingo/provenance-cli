use std::{fs, env, path::Path};

use clap::{Command, command, Arg};
use glob::glob;
use std::process;

use super::{config::Config, App};

pub fn cli(app: &mut App) {
    let matches = command!()
    .next_line_help(false)
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommand(
        Command::new("config")
        .arg_required_else_help(true)
        .about("Allows the user to view or modify the configuration.")
        .subcommand(Command::new("list")
            .about("Lists all the viewable and configurable settings")
        )
        .subcommand(Command::new("get")
            .about("Gets the value of a configuration setting")
            .arg(
                Arg::new("key")
                .required(true)
            )
        )
        .subcommand(Command::new("set")
            .about("Sets the value of a configuration setting")
            .arg(
                Arg::new("key")
                .short('k')
                .long("key")
                .required(true)

            )
            .arg(
                Arg::new("value")
                .short('v')
                .long("value")
                .required(true)
            )
        )
    )
    .subcommand(
        Command::new("action")
        .arg_required_else_help(true)
        .about("Runs a primitive action for Provenance")
        .subcommands(get_actions())
    )
    .subcommand(
        Command::new("function")
        .arg_required_else_help(true)
        .about("Runs a sequence of actions for Provenance")
        .subcommands(get_functions())
    )
    .subcommand(
        Command::new("scenario")
        .arg_required_else_help(true)
        .about("Runs a complex scenario for Provenance containing multiple functions")
        .subcommands(get_scenarios())
    )
    .get_matches();

    match matches.subcommand() {
        Some(("config", config_matches)) => {
            match config_matches.subcommand() {
                Some(("list", _list_matches)) => {
                    let keys = app.get_config().get_keys();
                    println!("{}", keys.join("\n"));
                },
                Some(("get", get_matches)) => {
                    let key = get_matches.get_one::<String>("key").unwrap();
                    let value = app.get_config().get(key).expect(format!("{} is not a valid key", key).as_str());
                    println!("{}", value);
                },
                Some(("set", set_matches)) => {
                    let key = set_matches.get_one::<String>("key").unwrap();
                    let value = set_matches.get_one::<String>("value").unwrap();
                    let success = app.get_config().set(key, value);
                    if !success {
                        println!("Unable to set {} to {}", key, value);
                    }
                    let path = app.state.config_path.clone();
                    app.get_config().save(&path);
                },
                _ => println!("Unreachable")
            }
        },
        Some(("action", action_matches)) => {
            match action_matches.subcommand() {
                Some((external, _)) => {
                    let shell = env::var("PIO_SCRIPT").expect("PIO_SCRIPT must be set");
                    let mut script_path = Path::new(&shell).join("scripts").join("actions");
                    script_path.push(format!("{}.sh", external));
                    run_script(script_path.to_string_lossy().to_string().as_str())
                }
                _ => println!("Unreachable")
            }
        },
        Some(("function", action_matches)) => {
            match action_matches.subcommand() {
                Some((external, _)) => {
                    let shell = env::var("PIO_SCRIPT").expect("PIO_SCRIPT must be set");
                    let mut script_path = Path::new(&shell).join("scripts").join("functions");
                    script_path.push(format!("{}.sh", external));
                    run_script(script_path.to_string_lossy().to_string().as_str())
                }
                _ => println!("Unreachable")
            }
        },
        Some(("scenario", action_matches)) => {
            match action_matches.subcommand() {
                Some((external, _)) => {
                    let shell = env::var("PIO_SCRIPT").expect("PIO_SCRIPT must be set");
                    let mut script_path = Path::new(&shell).join("scripts").join("scenarios");
                    script_path.push(format!("{}.sh", external));
                    run_script(script_path.to_string_lossy().to_string().as_str())
                }
                _ => println!("Unreachable")
            }
        }
        _ => println!("Unreachable")
    }
}

fn get_actions() -> Vec<Command> {
    let shell = env::var("PIO_SCRIPT").expect("PIO_SCRIPT must be set");
    let mut actions = Path::new(&shell).join("scripts").join("actions");
    actions.push("*");

    let mut commands = vec![];
    for entry in glob(actions.to_str().unwrap()).unwrap() {
        if let Ok(path) = entry {
            if let Some(file_name) = path.file_stem() {
                let file_name_str = file_name.to_string_lossy().to_string();
                commands.push(Command::new(file_name_str));
            }
        }
    }
    commands
}

fn get_functions() -> Vec<Command> {
    let shell = env::var("PIO_SCRIPT").expect("PIO_SCRIPT must be set");
    let mut actions = Path::new(&shell).join("scripts").join("functions");
    actions.push("*");

    let mut commands = vec![];
    for entry in glob(actions.to_str().unwrap()).unwrap() {
        if let Ok(path) = entry {
            if let Some(file_name) = path.file_stem() {
                let file_name_str = file_name.to_string_lossy().to_string();
                commands.push(Command::new(file_name_str));
            }
        }
    }
    commands
}

fn get_scenarios() -> Vec<Command> {
    let shell = env::var("PIO_SCRIPT").expect("PIO_SCRIPT must be set");
    let mut actions = Path::new(&shell).join("scripts").join("scenarios");
    actions.push("*");

    let mut commands = vec![];
    for entry in glob(actions.to_str().unwrap()).unwrap() {
        if let Ok(path) = entry {
            if let Some(file_name) = path.file_stem() {
                let file_name_str = file_name.to_string_lossy().to_string();
                commands.push(Command::new(file_name_str));
            }
        }
    }
    commands
}

fn run_script(script: &str) {
    let output = process::Command::new(script)
        .output()
        .expect("Failed to execute command");
    print!("{}", String::from_utf8_lossy(&output.stdout).to_string());
}