use monzo::{Client, Result};

use monzo_cli::*;

#[tokio::main]
async fn main() -> Result<()> {
    let cli_parameters = cli::parse();
    let access_token = get_access_token();
    let client = Client::new(access_token);

    let accounts = client.accounts().await?;

    let account_id = &accounts[0].id;

    match cli_parameters.subcommand {
        Some(cli::SubCommands::Pot) => {
            let pots = client.pots(account_id).await?;
            print_pots(pots);
        }
        Some(cli::SubCommands::Info) => print_account_info(accounts),
        Some(cli::SubCommands::Transactions) => {
            let transactions = get_transactions(client, account_id, cli_parameters).await?;
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
