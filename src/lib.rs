use std::env::var;
use monzo::{Balance, Pot};
use rusty_money::{iso, Money};

pub fn get_access_token() -> String {
    var("MONZO_ACCESS_TOKEN").unwrap_or_default()
}

pub fn print_pots(pots: Vec<Pot>) {
    pots.iter()
        .filter(|p| !p.deleted())
        .for_each(|p| {
            let pot_currency = iso::find(p.currency()).unwrap();
            let pot_bal = Money::from_minor(p.balance(), pot_currency);
            println!("{}: {}", p.name(), pot_bal);
        });
}

pub fn print_summary(balance: Balance, pots: Vec<Pot>) {
    let currency = iso::find(balance.currency()).unwrap();
    let formatted_balance = Money::from_minor(balance.balance(), currency);
    let total_balance = Money::from_minor(balance.total_balance(), currency);

    let open_pots = pots.iter().filter(|p| !p.deleted());

    println!("Current account balance: {}", formatted_balance);
    println!("Total balance: {}", total_balance);
    println!("Number of Pots: {}", open_pots.count());
}
