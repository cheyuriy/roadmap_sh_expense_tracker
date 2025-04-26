use super::models::{Transaction, Category, Limit};
use std::collections::HashMap;
use chrono::prelude::Utc;

pub fn summary(transactions: Vec<&Transaction>, month: Option<String>, category: Option<&Category>) -> (f64, HashMap<String, f64>) {
    if let Some(month) = month { 
        let transactions = transactions
            .iter()
            .filter(|&transaction| {
                if month == "overall" {
                    true
                } else {
                    transaction.datetime().format("%Y-%m").to_string() == month
                }
            })
            .filter(|&transaction| {
                if let Some(ref cat) = category {
                    transaction.category().as_ref() == Some(cat)
                } else {
                    true
                }
            })
            .collect::<Vec<_>>();

        let month_total = transactions.iter().map(|t| t.amount()).sum();
        let by_day_total = transactions.iter().fold(HashMap::new(), |mut acc, transaction| {
            let day = transaction.datetime().date_naive().format("%Y-%m-%d").to_string();
            *acc.entry(day).or_insert(0.0) += transaction.amount();
            acc
        });
        (month_total, by_day_total)
    } else {
        panic!("Invalid month format. Use YYYY-MM or 'overall'.");
    }
}

pub fn check_limit(transactions: Vec<&Transaction>, limit: Limit) -> f64 {
    let month = Utc::now().format("%Y-%m").to_string();
    let (total, _) = summary(transactions, Some(month), None);
    limit - total
}