use chrono::{Duration, Utc};
use monzo::inner_client::Refreshable;
use monzo::transactions::Transaction;
use monzo::{Account, Balance, Client, Pot, Result};
use rusty_money::{iso, Money};

pub mod auth;
pub mod cli;

use cli::Parameters;

pub fn print_account_info(accounts: Vec<Account>) {
    if accounts.is_empty() {
        eprintln!("There are no accounts");
        std::process::exit(1);
    }

    println!("{:<15}|{:<10}", "Account Number", "Sort Code");

    accounts.iter().for_each(|a| {
        let account_no = &a.account_number;
        let sort_code = &a.sort_code;
        let formatted_sort_code = format!(
            "{}-{}-{}",
            &sort_code[0..2],
            &sort_code[2..4],
            &sort_code[4..]
        );
        println!("{:<15}|{:<10}", account_no, formatted_sort_code);
    })
}

pub fn print_pots(pots: Vec<Pot>) {
    if pots.is_empty() {
        eprintln!("No pots available");
        std::process::exit(1);
    }

    println!("{:<10}|{:<10}", "Name", "Balance");

    pots.iter().filter(|p| !p.deleted).for_each(|p| {
        let pot_currency = iso::find(&p.currency).unwrap();
        let pot_bal = Money::from_minor(p.balance, pot_currency);
        println!("{:<10}|{:<10}", p.name, pot_bal);
    });
}

pub async fn get_transactions(
    client: Client<Refreshable>,
    account_id: &str,
    cli_parameters: Parameters,
) -> Result<Vec<Transaction>> {
    let cli_options = cli_parameters.options.unwrap();
    let since = cli_options.since;

    let transactions = client.transactions(account_id);

    let since_date = match since {
        0 => Utc::now(),
        d => Utc::now() - Duration::days(d as i64),
    };
    let transactions = transactions.since(since_date);

    let transactions = match cli_options.before {
        Some(b) => {
            let before_date = Utc::now() - Duration::days(b as i64);

            if before_date < since_date {
                eprintln!("Before date cannot be before since date");
                std::process::exit(1);
            }

            transactions.before(before_date)
        }
        None => transactions,
    };

    let result = transactions.send().await;

    result
}

pub fn print_transactions(transactions: Vec<Transaction>) {
    if transactions.is_empty() {
        eprintln!("No transactions available");
        std::process::exit(1);
    }

    println!(
        "{:<42}|{:<10}|{:<29}|{:<10}|{:<15}",
        "Description", "Category", "Date", "Amount", "Notes"
    );

    transactions.iter().rev().for_each(|t| {
        let transaction_currency = iso::find(&t.currency).unwrap();
        println!(
            "{:<42}|{:<10}|{:<29}|{:<10}|{:<15}",
            t.description,
            t.category,
            t.created.to_string(),
            Money::from_minor(t.amount, transaction_currency).to_string(),
            t.notes
        );
    });
}

pub fn print_summary(balance: Balance, pots: Vec<Pot>) {
    let currency = iso::find(&balance.currency).unwrap();
    let formatted_balance = Money::from_minor(balance.balance, currency);
    let total_balance = Money::from_minor(balance.total_balance, currency);
    let spend_today = Money::from_minor(balance.spend_today, currency);

    let open_pots = pots.iter().filter(|p| !p.deleted);

    println!(
        "{:<25}{:<10}",
        "Current account balance:", formatted_balance
    );
    println!("{:<25}{:<10}", "Total balance:", total_balance);
    println!("{:<25}{:<10}", "Spend today:", spend_today);
    println!("{:<25}{:<10}", "Number of Pots:", open_pots.count());
}
