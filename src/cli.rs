use clap::{App, AppSettings, Arg, SubCommand};

pub struct Parameters {
    pub subcommand: Option<SubCommands>,
    pub options: Option<CommandOptions>,
}

pub enum SubCommands {
    Info,
    Pot,
    Transactions,
}

pub enum CommandOptions {
    List,
}

pub fn parse() -> Parameters {
    let matches = App::new("monzo")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DisableVersion)
        .subcommand(SubCommand::with_name("info").about("Information about your account"))
        .subcommand(
            SubCommand::with_name("pot")
                .about("Interact with your Monzo pots")
                .arg(
                    Arg::with_name("pot-list")
                        .short("l")
                        .required(true)
                        .long("list"),
                ),
        )
        .subcommand(
            SubCommand::with_name("transactions").about("View transactions from the last 7 days"),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("info") => Parameters {
            subcommand: Some(SubCommands::Info),
            options: None,
        },
        Some("pot") => {
            if matches.is_present("pot-list") {
                Parameters {
                    subcommand: Some(SubCommands::Pot),
                    options: Some(CommandOptions::List),
                }
            } else {
                Parameters {
                    subcommand: None,
                    options: None,
                }
            }
        }
        Some("transactions") => Parameters {
            subcommand: Some(SubCommands::Transactions),
            options: None,
        },
        Some(_) => panic!("Unrecognised subcommand"),
        None => Parameters {
            subcommand: None,
            options: None,
        },
    }
}
