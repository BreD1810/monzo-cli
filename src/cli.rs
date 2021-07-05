use clap::{Arg, App, AppSettings, SubCommand};

pub struct Parameters {
    pub subcommand: Option<SubCommands>,
    pub options: Option<CommandOptions>,
}

pub enum SubCommands {
    Pot,
}

pub enum CommandOptions {
    List,
}

pub fn parse() -> Parameters {
    let matches = App::new("monzo")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DisableVersion)
        .subcommand(SubCommand::with_name("pot")
            .about("Interact with your Monzo pots")
            .arg(Arg::with_name("pot-list")
                .short("l")
                .required(true)
                .long("list")))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("pot") {
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
    } else {
        Parameters {
            subcommand: None,
            options: None,
        }
    }
}
