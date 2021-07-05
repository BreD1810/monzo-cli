use clap::{App, AppSettings, Arg, SubCommand};

pub struct Parameters {
    pub subcommand: Option<SubCommands>,
    pub options: Option<CommandOptions>,
}

pub enum SubCommands {
    Pot,
    Info,
}

pub enum CommandOptions {
    List,
}

pub fn parse() -> Parameters {
    let matches = App::new("monzo")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DisableVersion)
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
        .subcommand(SubCommand::with_name("info").about("Information about your account"))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("pot") {
        if matches.is_present("pot-list") {
            return Parameters {
                subcommand: Some(SubCommands::Pot),
                options: Some(CommandOptions::List),
            };
        } else {
            return Parameters {
                subcommand: None,
                options: None,
            };
        }
    }

    match matches.subcommand_matches("info") {
        Some(_) => Parameters {
            subcommand: Some(SubCommands::Info),
            options: None,
        },
        None => Parameters {
            subcommand: None,
            options: None,
        },
    }
}
