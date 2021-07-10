use clap::{value_t, App, AppSettings, Arg, SubCommand};

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
    Since(i64),
}

pub fn parse() -> Parameters {
    let matches = App::new("monzo")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DisableVersion)
        .subcommand(SubCommand::with_name("info").about("Information about your account"))
        .subcommand(SubCommand::with_name("pots").about("List your Monzo pots"))
        .subcommand(
            SubCommand::with_name("transactions")
                .about("View transactions from the last 7 days")
                .arg(
                    Arg::with_name("transaction-since")
                        .long("since")
                        .help("Number or days ago to list transactions from")
                        .default_value("7"),
                ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("info") => Parameters {
            subcommand: Some(SubCommands::Info),
            options: None,
        },
        Some("pots") => Parameters {
            subcommand: None,
            options: None,
        },
        Some("transactions") => {
            let days = value_t!(matches.value_of("transaction-since"), i64).unwrap_or(7);
            Parameters {
                subcommand: Some(SubCommands::Transactions),
                options: Some(CommandOptions::Since(days)),
            }
        }
        Some(_) => panic!("Unrecognised subcommand"),
        None => Parameters {
            subcommand: None,
            options: None,
        },
    }
}
