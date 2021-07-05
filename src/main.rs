use monzo::{Client, Result};

use monzo_cli::*;
use cli::SubCommands;

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli_parameters = cli::parse();
    let access_token = get_access_token();
    let client = Client::quick(access_token);

    let accounts = client.accounts().await?;

    let account_id = accounts[0].id();

    match cli_parameters.subcommand {
        Some(SubCommands::Pot) => {
            let pots = client.pots(account_id).await?;
            print_pots(pots);
        },
        None => {
            let balance = client.balance(account_id).await?;
            let pots = client.pots(account_id).await?;
            print_summary(balance, pots);
        },
    }

    Ok(())
}
