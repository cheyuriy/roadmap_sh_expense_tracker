mod cli;
mod models;
mod store;
mod summary;

use std::iter::once;
use std::collections::HashMap;
use cli::CLI;
use clap::Parser;
use store::Store;
use models::{Transaction, Category};
use summary::{summary, check_limit};
use tabled::{builder::Builder, settings::Style};
use csv::Writer;

fn main() {
    let cli = CLI::parse();
    let mut store = Store::new(None);

    match cli.commands {
        cli::Commands::Add { description, amount, category } => {
            let category = if let Some(category_id) = category {
                Some(store.get_category(category_id).expect("Category not found"))
            } else {
                None
            };

            let id = store.add_transaction(description, amount, category);
            println!("Added transaction with ID: {:?})", id);
            let limit = store.limit();
            if let Some(limit) = limit {
                let remaining = check_limit(store.list_transactions(None), limit);
                if remaining < 0.0 {
                    println!("Spending limit of {:?} exceeded!", limit);
                }
            }
        }
        cli::Commands::Delete { id } => {
            store.delete_transaction(id);
            println!("Deleted transaction with ID: {}", id);
        }
        cli::Commands::List { category } => {
            let category = if let Some(category_id) = category {
                Some(store.get_category(category_id).expect("Category not found"))
            } else {
                None
            };
            let transactions = store.list_transactions(category);
            let table = create_table_transactions(transactions);
            println!("{}", table);
        }
        cli::Commands::Summary { month, category} => {
            let category = if let Some(category_id) = category {
                Some(store.get_category(category_id).expect("Category not found"))
            } else {
                None
            };
            let (total, by_day) = summary(store.list_transactions(None), Some(month.clone()), category.as_ref());
            if month == "overall" {
                println!("Showing summary for all transactions:");
            } else {
                println!("Showing summary for month {:?}:", month);
            }
            let table = create_table_by_day(by_day, total);
            println!("{}", table);
        }
        cli::Commands::Limit { amount } => {
            store.set_limit(amount);
            if amount == 0.0 {
                println!("Removed spending limit.");
            } else {
                println!("Set spending limit to: {:?}", amount);
            };
        }
        cli::Commands::Export { filename } => {
            let transactions = store.list_transactions(None);
            let mut wtr = Writer::from_path(filename.clone()).expect("Unable to create CSV writer");
            for transaction in transactions {
                wtr.write_record(&[
                    transaction.id().to_string(),
                    transaction.description().to_string(),
                    transaction.amount().to_string(),
                    transaction.datetime().to_string(),
                    transaction.category().map_or("None".to_string(), |cat| cat.name().to_string()),
                ]).expect("Unable to write record");
            }
            wtr.flush().expect("Unable to flush CSV writer");
            println!("Exporting transactions to: {}", filename);
        }
        cli::Commands::Category { category_subcommand } => match category_subcommand {
            cli::CategorySubcommand::Add { name } => {
                let id = store.add_category(&name);
                println!("Added category with ID: {:?})", id);
            },
            cli::CategorySubcommand::Delete { id } => {
                store.delete_category(id);
                println!("Deleted category with ID: {}", id);
            },
            cli::CategorySubcommand::List => {  
                let categories = store.list_categories();
                let table = create_table_categories(categories);
                println!("{}", table);
            },
        },
        
    }
}

fn create_table_transactions(transactions: Vec<&Transaction>) -> String {
    let mut builder = Builder::default();
    for transaction in transactions {
        builder.push_record(vec![
            transaction.id().to_string(),
            transaction.description().to_string(),
            transaction.amount().to_string(),
            transaction.datetime().to_string(),
            transaction.category().map_or("None".to_string(), |cat| cat.name().to_string()),
        ]);
    }
    let headers = once(String::new()).chain(
        ["Description", "Amount", "Datetime", "Category"].map(|i| i.to_string())
    );
    builder.insert_record(0, headers);
    builder.build().with(Style::modern()).to_string()
}

fn create_table_categories(categories: Vec<&Category>) -> String {
    let mut builder = Builder::default();
    for category in categories {
        builder.push_record(vec![
            category.id().to_string(),
            category.name().to_string(),
        ]);
    }
    let headers = once(String::new()).chain(
        ["Name"].map(|i| i.to_string())
    );
    builder.insert_record(0, headers);
    builder.build().with(Style::modern()).to_string()
}

fn create_table_by_day(by_day: HashMap<String, f64>, total: f64) -> String {
    let mut builder = Builder::default();

    let mut by_day_vec: Vec<_> = by_day.iter().collect();
    by_day_vec.sort_by(|a, b| a.0.cmp(b.0));

    for (day, total) in by_day_vec {
        builder.push_record(vec![
            day.to_string(),
            total.to_string(),
        ]);
    }
    let headers = once(String::new()).chain(
        ["Amount"].map(|i| i.to_string())
    );
    builder.insert_record(0, headers);
    builder.push_record(vec![
        "Total".to_string(),
        total.to_string(),
    ]);
    builder.build().with(Style::modern()).to_string()
}