use chrono::{Duration, Utc};
use monzo::inner_client::Refreshable;
use monzo::transactions::Transaction;
use monzo::{Account, Balance, Client, Pot, Result};
use rusty_money::{iso, Money};

pub mod auth;
pub mod cli;

use cli::CommandOptions;

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
    client: &Client<Refreshable>,
    account_id: &str,
    cli_options: &CommandOptions,
) -> Result<Vec<Transaction>> {
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

pub fn print_transactions(transactions: Vec<Transaction>, pots: Vec<Pot>, include_declined: bool) {
    if transactions.is_empty() {
        eprintln!("No transactions available");
        std::process::exit(1);
    }

    if include_declined {
        print_transactions_with_declined(transactions, pots);
    } else {
        print_transactions_without_declined(transactions, pots);
    }
}

fn print_transactions_without_declined(transactions: Vec<Transaction>, pots: Vec<Pot>) {
    println!(
        "{:<42}|{:<15}|{:<29}|{:<10}|{:<15}",
        "Description", "Category", "Date", "Amount", "Notes"
    );

    transactions.iter().rev().for_each(|t| {
        if t.decline_reason.is_some() {
            return;
        }

        let transaction_currency = iso::find(&t.currency).unwrap();
        let description = get_transaction_description(t.description.clone(), &pots);
        println!(
            "{:<42}|{:<15}|{:<29}|{:<10}|{:<15}",
            description,
            t.category,
            t.created.to_string(),
            Money::from_minor(t.amount, transaction_currency).to_string(),
            t.notes.replace("\n", " ")
        );
    });
}

fn print_transactions_with_declined(transactions: Vec<Transaction>, pots: Vec<Pot>) {
    println!(
        "{:<42}|{:<15}|{:<29}|{:<10}|{:<10}|{:<15}",
        "Description", "Category", "Date", "Amount", "Declined?", "Notes"
    );

    transactions.iter().rev().for_each(|t| {
        let declined = t.decline_reason.is_some();
        let transaction_currency = iso::find(&t.currency).unwrap();
        let description = get_transaction_description(t.description.clone(), &pots);
        println!(
            "{:<42}|{:<15}|{:<29}|{:<10}|{:<10}|{:<15}",
            description,
            t.category,
            t.created.to_string(),
            Money::from_minor(t.amount, transaction_currency).to_string(),
            declined,
            t.notes.replace("\n", " ")
        );
    });
}

fn get_transaction_description(description: String, pots: &[Pot]) -> String {
    let matching_pots: Vec<&Pot> = pots.iter().filter(|p| p.id == description).collect();

    if matching_pots.len() == 1 {
        format!("POT - {}", matching_pots.first().unwrap().name)
    } else {
        description
    }
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
