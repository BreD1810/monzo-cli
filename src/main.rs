use monzo::Result;

use monzo_cli::*;

#[tokio::main]
async fn main() -> Result<()> {
    let cli_parameters = cli::parse();

    match cli_parameters.subcommand {
        Some(cli::SubCommands::Auth) => {
            auth::auth().await;
        }
        Some(cli::SubCommands::Pots) => {
            let client = auth::get_authed_client().await;
            let accounts = client.accounts().await?;
            let account_id = &accounts[0].id;
            let pots = client.pots(account_id).await?;
            print_pots(pots);
        }
        Some(cli::SubCommands::Info) => {
            let client = auth::get_authed_client().await;
            let accounts = client.accounts().await?;
            print_account_info(accounts);
        }
        Some(cli::SubCommands::Transactions) => {
            let client = auth::get_authed_client().await;
            let accounts = client.accounts().await?;
            let account_id = &accounts[0].id;
            let cli_options = cli_parameters.options.unwrap();
            let transactions = get_transactions(&client, account_id, &cli_options).await?;
            let pots = client.pots(account_id).await?;
            print_transactions(transactions, pots, cli_options.include_declined);
        }
        None => {
            let client = auth::get_authed_client().await;
            let accounts = client.accounts().await?;
            let account_id = &accounts[0].id;
            let balance = client.balance(account_id).await?;
            let pots = client.pots(account_id).await?;
            print_summary(balance, pots);
        }
    }

    Ok(())
}
