use monzo::transactions::Transaction;
use monzo::{Account, Balance, Pot};
use rusty_money::{iso, Money};
use std::env::var;
use std::env::VarError;

pub fn get_access_token() -> String {
    match var("MONZO_ACCESS_TOKEN") {
        Ok(t) => t,
        Err(e) if e == VarError::NotPresent => {
            eprintln!("Error: `MONZO_ACCESS_TOKEN` environment variable is not set.");
            std::process::exit(1);
        }
        Err(e) => std::panic::panic_any(e),
    }
}

pub fn print_account_info(accounts: Vec<Account>) {
    accounts.iter().for_each(|a| {
        let account_no = &a.account_number;
        let sort_code = &a.sort_code;

        println!("Account Number:\t{}", account_no);
        println!(
            "Sort code:\t{}-{}-{}",
            &sort_code[0..2],
            &sort_code[2..4],
            &sort_code[4..]
        );
    })
}

pub fn print_pots(pots: Vec<Pot>) {
    pots.iter().filter(|p| !p.deleted).for_each(|p| {
        let pot_currency = iso::find(&p.currency).unwrap();
        let pot_bal = Money::from_minor(p.balance, pot_currency);
        println!("{}:\t{}", p.name, pot_bal);
    });
}

pub fn print_transactions(transactions: Vec<Transaction>) {
    transactions.iter().for_each(|t| {
        let transaction_currency = iso::find(&t.currency).unwrap();
        println!("Description:\t{}", t.description);
        println!("Category:\t{}", t.category);
        println!("Date:\t{}", t.created.to_string());
        println!(
            "Amount:\t{}",
            Money::from_minor(t.amount, transaction_currency)
        );
        println!("Notes:\t{}", t.notes);
        println!();
    });
}

pub fn print_summary(balance: Balance, pots: Vec<Pot>) {
    let currency = iso::find(&balance.currency).unwrap();
    let formatted_balance = Money::from_minor(balance.balance, currency);
    let total_balance = Money::from_minor(balance.total_balance, currency);

    let open_pots = pots.iter().filter(|p| !p.deleted);

    println!("Current account balance:\t{}", formatted_balance);
    println!("Total balance:\t\t\t{}", total_balance);
    println!("Number of Pots:\t\t\t{}", open_pots.count());
}
