use chrono::{Duration, Utc};
use monzo::{Client, Result};

use cli::SubCommands;
use monzo_cli::*;

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli_parameters = cli::parse();
    let access_token = get_access_token();
    let client = Client::new(access_token);

    let accounts = client.accounts().await?;

    let account_id = &accounts[0].id;

    match cli_parameters.subcommand {
        Some(SubCommands::Pot) => {
            let pots = client.pots(account_id).await?;
            print_pots(pots);
        }
        Some(SubCommands::Info) => print_account_info(accounts),
        Some(SubCommands::Transactions) => {
            let transactions = client
                .transactions(account_id)
                .since(Utc::now() - Duration::days(7))
                .send()
                .await?;

            print_transactions(transactions);
        }
        None => {
            let balance = client.balance(account_id).await?;
            let pots = client.pots(account_id).await?;
            print_summary(balance, pots);
        }
    }

    Ok(())
}
