use clap::{Command, command, Arg};

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
        _ => println!("Unreachable")
    }
}