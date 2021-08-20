use clap::{value_t, App, AppSettings, Arg, SubCommand};

pub struct Parameters {
    pub subcommand: Option<SubCommands>,
    pub options: Option<CommandOptions>,
}

pub enum SubCommands {
    Auth,
    Info,
    Pots,
    Transactions,
}

pub struct CommandOptions {
    pub since: usize,
    pub before: Option<usize>,
    pub include_declined: bool,
}

pub fn parse() -> Parameters {
    let matches = App::new("monzo")
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DisableVersion)
        .subcommand(SubCommand::with_name("auth").about("Authenticate your Monzo account"))
        .subcommand(SubCommand::with_name("info").about("Information about your account"))
        .subcommand(SubCommand::with_name("pots").about("List your Monzo pots"))
        .subcommand(
            SubCommand::with_name("transactions")
                .about("View transactions from the last 7 days")
                .arg(
                    Arg::with_name("transaction-since")
                        .long("since")
                        .help("Number of days ago to list transactions from")
                        .default_value("7"),
                )
                .arg(
                    Arg::with_name("transaction-before")
                        .long("before")
                        .help("Number of days ago to list transactions before")
                        .default_value("0"),
                )
                .arg(
                    Arg::with_name("include-declined")
                        .long("include-declined")
                        .help("Include declined transactions")
                        .takes_value(false)
                ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("auth") => Parameters {
            subcommand: Some(SubCommands::Auth),
            options: None,
        },
        Some("info") => Parameters {
            subcommand: Some(SubCommands::Info),
            options: None,
        },
        Some("pots") => Parameters {
            subcommand: Some(SubCommands::Pots),
            options: None,
        },
        Some("transactions") => {
            let submatch = matches.subcommand_matches("transactions").unwrap();
            let days = value_t!(submatch.value_of("transaction-since"), usize).unwrap();
            let before = match value_t!(submatch.value_of("transaction-before"), usize) {
                Ok(b) => Some(b),
                Err(_) => None,
            };
            let include_declined = submatch.is_present("include-declined");
            Parameters {
                subcommand: Some(SubCommands::Transactions),
                options: Some(CommandOptions {
                    since: days,
                    before,
                    include_declined,
                }),
            }
        }
        Some(_) => panic!("Unrecognised subcommand"),
        None => Parameters {
            subcommand: None,
            options: None,
        },
    }
}
