mod cli;
mod models;
mod store;

use cli::CLI;
use clap::Parser;
use models::Limit;
use store::Store;

fn main() {
    let cli = CLI::parse();
    println!("{:?}", cli);

    let mut store = Store::new(None);

    match cli.commands {
        cli::Commands::Add { description, amount, category } => {
            let category = if let Some(category_id) = category {
                Some(store.get_category(category_id).expect("Category not found"))
            } else {
                None
            };

            store.add_transaction(description, amount, category);
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
            println!("Listing all categories:");
            println!("{:?}", transactions);
        }
        cli::Commands::Summary { month } => {
            println!("Showing summary for month: {}", month);
        }
        cli::Commands::Limit { amount } => {
            let limit = Limit::new(amount);
            println!("Set spending limit to: {:?}", limit);
        }
        cli::Commands::Export { filename } => {
            println!("Exporting transactions to: {}", filename);
        }
        cli::Commands::Category { category_subcommand } => match category_subcommand {
            cli::CategorySubcommand::Add { name } => {
                let id = store.add_category(&name);
                println!("Added category: {:?} (ID: {:?})", name, id);
            },
            cli::CategorySubcommand::Delete { id } => {
                store.delete_category(id);
                println!("Deleted category with ID: {}", id);
            },
            cli::CategorySubcommand::List => {  
                let categories = store.list_categories();
                println!("Listing all categories:");
                println!("{:?}", categories);
            },
        },
        
    }
}
