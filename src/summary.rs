use super::models::{Transaction, Category, Limit};
use std::collections::HashMap;
use chrono::prelude::Utc;

/// Function to create a summary of transactions for a given month or overall, and optionally filter by category.
/// It returns the total amount and a breakdown by day.
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

/// Function to check the remaining limit for the current month.
/// It takes a vector of transactions and a limit, and returns the remaining amount.
/// If the limit is exceeded, it returns a negative value.
pub fn check_limit(transactions: Vec<&Transaction>, limit: Limit) -> f64 {
    let month = Utc::now().format("%Y-%m").to_string();
    let (total, _) = summary(transactions, Some(month), None);
    limit - total
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::Utc;

    #[test]
    fn test_summary_overall() {
        let curr_day = Utc::now().date_naive().format("%Y-%m-%d").to_string();
        let transactions = vec![
            Transaction::new(1, 100.0, "Test transaction 1".to_string(), None),
            Transaction::new(2, 200.0, "Test transaction 2".to_string(), None),
        ];
        let (total, by_day) = summary(transactions.iter().collect(), Some("overall".to_string()), None);
        assert_eq!(total, 300.0);
        assert_eq!(by_day.len(), 1);
        assert_eq!(by_day.get(&curr_day).unwrap(), &300.0);
    }
    #[test]
    fn test_summary_month() {
        let curr_day = Utc::now().date_naive().format("%Y-%m-%d").to_string();
        let curr_month = Utc::now().format("%Y-%m").to_string();
        let transactions = vec![
            Transaction::new(1, 100.0, "Test transaction 1".to_string(), None),
            Transaction::new(2, 200.0, "Test transaction 2".to_string(), None),
        ];
        let (total, by_day) = summary(transactions.iter().collect(), Some(curr_month), None);
        assert_eq!(total, 300.0);
        assert_eq!(by_day.len(), 1);
        assert_eq!(by_day.get(&curr_day).unwrap(), &300.0); 
    }

    #[test]
    fn test_summary_month_with_category() {
        let curr_day = Utc::now().date_naive().format("%Y-%m-%d").to_string();
        let curr_month = Utc::now().format("%Y-%m").to_string();
        let category = Category::new(1, "Food".to_string());
        let transactions = vec![
            Transaction::new(1, 100.0, "Test transaction 1".to_string(), Some(category.clone())),
            Transaction::new(2, 200.0, "Test transaction 2".to_string(), None),
        ];
        let (total, by_day) = summary(transactions.iter().collect(), Some(curr_month), Some(&category));
        assert_eq!(total, 100.0);
        assert_eq!(by_day.len(), 1);
        assert_eq!(by_day.get(&curr_day).unwrap(), &100.0);
    }

    #[test]
    fn test_check_limit() {
        let transactions = vec![
            Transaction::new(1, 100.0, "Test transaction 1".to_string(), None),
            Transaction::new(2, 200.0, "Test transaction 2".to_string(), None),
        ];
        let limit = 500.0;
        let remaining = check_limit(transactions.iter().collect(), limit);
        assert_eq!(remaining, 200.0);
        let limit = 100.0;
        let remaining = check_limit(transactions.iter().collect(), limit);
        assert_eq!(remaining, -200.0);
    }
}