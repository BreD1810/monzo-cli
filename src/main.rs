use std::env;
use monzo::{Client, Result};
use rusty_money;

#[tokio::main]
async fn main() -> Result<()> {
    let access_token = env::var("MONZO_ACCESS_TOKEN").unwrap_or_default();
    let client = Client::quick(access_token);

    let accounts = client.accounts().await?;

    let account_id = accounts[0].id();
    let balance = client.balance(account_id).await?;

    let currency = rusty_money::iso::find(balance.currency()).unwrap();

    let formatted_balance = rusty_money::Money::from_minor(balance.balance(), currency);

    let total_balance = rusty_money::Money::from_minor(balance.total_balance(), currency);

    println!("Your balance is: {}", formatted_balance);
    println!("Total balance: {}", total_balance);
    println!("Number of Accounts: {}", accounts.len());
    Ok(())
}
