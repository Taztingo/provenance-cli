use std::{fs, env, path::{Path, PathBuf}};

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
                    run_script(script_path.to_string_lossy().to_string().as_str(), &app.config)
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
                    run_script(script_path.to_string_lossy().to_string().as_str(), &app.config)
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
                    run_script(script_path.to_string_lossy().to_string().as_str(), &app.config)
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
            commands.push(script_to_command(path));
        }
    }
    commands
}

fn run_script(script: &str, config: &Config) {
    let output = process::Command::new(script)
        .arg(&config.provenance_build)
        .arg(&config.provenance_binary)
        .arg(&config.provenance_home)
        .arg("20")
        .output()
        .expect("Failed to execute command");
    print!("{}", String::from_utf8_lossy(&output.stdout).to_string());
}

struct Script {
    name: String,
    author: String,
    version: String,
    description: String,
    args: Vec<ScriptArg>,
}

impl Script {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            author: "".to_string(),
            version: "".to_string(),
            description: "".to_string(),
            args: vec![],
        }
    }
}

struct ScriptArg {
    pub name: String,
    pub description: String,
    pub default: String,
}

fn read_script(filepath: &PathBuf) -> Script {
    let file_contents = fs::read_to_string(filepath).expect("Unable to read file");
    let mut script = Script::new();

    if let Some(file_name) = filepath.file_stem() {
        script.name = file_name.to_string_lossy().to_string();
    }

    // Parse the file
    let mut parse_description = false;
    let mut parse_author = false;
    let mut parse_version = false;
    let mut parse_args = false;
    for line in file_contents.lines() {
        // Check for end parsing
        let line = line.trim();
        if line == "#" {
            parse_description = false;
            parse_author = false;
            parse_version = false;
            parse_args = false;
        }

        // Check for content parsing
        if parse_description {
            // Remove the "# " from the string and the rest is the description
            script.description = line.replace("# ", "");
        } else if parse_author {
            // Remove the "# " from the string and the rest is the author
            script.author = line.replace("# ", "");
        } else if parse_version {
            // Remove the "# " from the string and the rest is the version
            script.version = line.replace("# ", "");
        } else if parse_args {
            // Remove the "# " from the string.
            let arg = line.replace("# ", "");

            // Split the string by ;
            // First item is the arg name
            // Second item is the arg description
            // Third item is the default value
            let mut split = arg.split(";");
            script.args.push(ScriptArg{
                name: split.next().unwrap().to_string(),
                description: split.next().unwrap().to_string(),
                default: split.next().unwrap().to_string()
            });
        }

        // Check for start parsing
        if line.contains("# DESCRIPTION") {
            parse_description = true;
        } else if line.contains("# AUTHOR") {
            parse_author = true;
        } else if line.contains("# VERSION") {
            parse_version = true;
        } else if line.contains("# ARGS") {
            parse_args = true;
        }
    }

    script
}

fn script_to_command(path: PathBuf) -> Command {
    let script = read_script(&path);
    let mut command = Command::new(script.name)
        .about(script.description)
        .author(script.author)
        .version(script.version);
    for arg in &script.args {
        command = command.arg(
            Arg::new(&arg.name)
            .help(&arg.description)
            .required(true)
        );
    }
    command
}