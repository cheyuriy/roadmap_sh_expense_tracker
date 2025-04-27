use super::models::{Transaction, Category, TransactionId, CategoryId, Limit};
use serde::{Serialize, Deserialize};
use std::{fs, vec};
use std::io::Write;
use std::path::Path;

/// Store struct to manage transactions and categories
/// It contains methods to add, delete, list transactions and categories
/// and to set a spending limit.
/// It also handles the persistence of data to a JSON file.
/// The data is stored in a JSON file, and the struct is initialized
/// with the data from the file if it exists.
/// If the file does not exist, it creates an empty file and initializes the struct.
#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    transactions: Vec<Transaction>,

    #[serde(skip)]
    max_transaction_id: TransactionId,

    categories: Vec<Category>,

    #[serde(skip)]
    max_category_id: CategoryId,

    limit: Option<Limit>,

    #[serde(skip)]
    path: String
}

impl Store {
    /// Creates a new Store instance.
    /// If a file path is provided, it will be used to load the data.
    /// If no file path is provided, it will default to "data/data.json".
    pub fn new(file_path: Option<&str>) -> Self {
        let path = if let Some(p) = file_path  {
            p 
        } else {
            "data/data.json"            
        };
        if let Ok(is_exists) = fs::exists(path) {
            if is_exists {
                let data = fs::read_to_string(path).expect("Unable to read file");
                let mut store: Store = serde_json::from_str(&data).expect("Unable to parse JSON");
                store.max_transaction_id = store.transactions.iter().map(|i| i.id()).max().unwrap_or(0);
                store.max_category_id = store.categories.iter().map(|i| i.id()).max().unwrap_or(0);
                store.path = path.to_string();
                store
            } else {
                let empty_transactions: Vec<Transaction> = vec![];
                let empty_categories: Vec<Category> = vec![];
                let s = Store {
                    transactions: empty_transactions,
                    max_transaction_id: 0,
                    categories: empty_categories,
                    max_category_id: 0,
                    limit: None,
                    path: path.to_string()
                };
                s.persist();
                s
            }
        } else {
            panic!("Can't check existence of file `data.json`");
        }
    }

    /// Persists the current state of the Store to a JSON file.
    fn persist(&self) {
        let json = serde_json::to_string_pretty(&self).expect("Unable to write JSON");

        let path = Path::new(&self.path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Unable to create directory");
        }
        let mut file = fs::File::create(&self.path).expect("Unable to create file");
        file.write_all(json.as_bytes()).expect("Unable to write file");
    }

    /// Adds a new transaction to the store.
    /// It returns the ID of the newly created transaction.
    pub fn add_transaction(&mut self, description: String, amount: f64, category: Option<Category>) -> TransactionId{
        let transaction = Transaction::new(
            self.max_transaction_id + 1,
            amount,
            description,
            category
        );
        self.transactions.push(transaction);
        self.max_transaction_id += 1;
        self.persist();
        self.max_transaction_id
    }

    /// Deletes a transaction from the store.
    pub fn delete_transaction(&mut self, id: TransactionId) {
        if let Some(pos) = self.transactions.iter().position(|transaction| transaction.id() == id) {
            self.transactions.remove(pos);
            self.persist();
        }
    }

    /// Lists all transactions in the store.
    /// If a category is provided, it filters the transactions by that category, otherwise it lists all transactions.
    /// The transactions are sorted by their datetime in ascending order.
    pub fn list_transactions(&self, category: Option<Category>) -> Vec<&Transaction> {
        let mut transactions: Vec<&Transaction> = if let Some(_) = category {
            self.transactions.iter().filter(|&transaction| transaction.category() == category).collect()
        } else {
            self.transactions.iter().collect()
        };
        transactions.sort_by(|a, b| a.datetime().cmp(&b.datetime()));
        transactions
    }

    /// Returns a clone of the category by its ID.
    /// If the category is not found, it returns None.
    pub fn get_category(&self, id: CategoryId) -> Option<Category> {
        if let Some(cat) = self.categories.iter().find(|&cat| cat.id() == id) {
            Some(cat.clone())
        } else {
            None    
        }
    }

    /// Adds a new category to the store.
    /// It returns the ID of the newly created category.    
    pub fn add_category(&mut self, name: &str) -> CategoryId {
        let category = Category::new(
            self.max_category_id + 1,
            name.to_string()
        );
        self.categories.push(category);
        self.max_category_id += 1;
        self.persist();
        self.max_category_id
    }

    /// Deletes a category from the store.
    /// If the category is used in any transaction, it will be removed from that transaction.
    /// If the category is not found, it does nothing.
    pub fn delete_category(&mut self, id: CategoryId) {
        if let Some(pos) = self.categories.iter().position(|cat| cat.id() == id) {
            self.categories.remove(pos);
        }
        for transaction in self.transactions.iter_mut() {
            if transaction.category().is_some() && transaction.category().unwrap().id() == id {
                transaction.remove_category();
            }
        }
        self.persist();
    }

    /// Lists all categories in the store.
    pub fn list_categories(&self) -> Vec<&Category> {
        self.categories.iter().collect()
    }

    /// Sets a spending limit for the current month.
    /// If the limit is set to 0, it removes the limit.
    pub fn set_limit(&mut self, limit: f64) {
        if limit > 0.0 {
            self.limit = Some(limit);
        } else {
            self.limit = None;
        }
        self.persist();
    }

    /// Returns the current spending limit.
    /// If no limit is set, it returns None.
    pub fn limit(&self) -> Option<Limit> {
        self.limit
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::Utc;

    #[test]
    fn test_add_transaction() {
        let temp_file = "test_data_1.json";
        let mut store = Store::new(Some(temp_file));
        let id = store.add_transaction("Test transaction".to_string(), 100.0, None);
        assert_eq!(store.transactions.len(), 1);
        assert_eq!(store.transactions[0].id(), id);
        assert_eq!(store.transactions[0].description(), "Test transaction");
        assert_eq!(store.transactions[0].amount(), 100.0);
        assert_eq!(store.transactions[0].datetime().date_naive(), Utc::now().date_naive());
        fs::remove_file(temp_file).expect("Unable to remove file");
    }

    #[test]
    fn test_delete_transaction() {
        let temp_file = "test_data_2.json";
        let mut store = Store::new(Some(temp_file));
        let id = store.add_transaction("Test transaction".to_string(), 100.0, None);
        store.delete_transaction(id);
        assert_eq!(store.transactions.len(), 0);
        fs::remove_file(temp_file).expect("Unable to remove file");
    }

    #[test]
    fn test_list_transactions() {
        let temp_file = "test_data_3.json";
        let mut store = Store::new(Some(temp_file));
        store.add_transaction("Test transaction 1".to_string(), 100.0, None);
        store.add_transaction("Test transaction 2".to_string(), 200.0, None);
        let transactions = store.list_transactions(None);
        assert_eq!(transactions.len(), 2);
        assert_eq!(transactions[0].description(), "Test transaction 1");
        assert_eq!(transactions[1].description(), "Test transaction 2");
        fs::remove_file(temp_file).expect("Unable to remove file");
    }

    #[test]
    fn test_add_category() {
        let temp_file = "test_data_4.json";
        let mut store = Store::new(Some(temp_file));
        let id = store.add_category("Test category");
        assert_eq!(store.categories.len(), 1);
        assert_eq!(store.categories[0].id(), id);
        assert_eq!(store.categories[0].name(), "Test category");
        fs::remove_file(temp_file).expect("Unable to remove file");
    }

    #[test]
    fn test_delete_category() {
        let temp_file = "test_data_5.json";
        let mut store = Store::new(Some(temp_file));
        let id = store.add_category("Test category");
        store.delete_category(id);
        assert_eq!(store.categories.len(), 0);
        assert_eq!(store.transactions.len(), 0);
        fs::remove_file(temp_file).expect("Unable to remove file");
    }

    #[test]
    fn test_list_categories() {
        let temp_file = "test_data_6.json";
        let mut store = Store::new(Some(temp_file));
        store.add_category("Test category 1");
        store.add_category("Test category 2");
        let categories = store.list_categories();
        assert_eq!(categories.len(), 2);
        assert_eq!(categories[0].name(), "Test category 1");
        assert_eq!(categories[1].name(), "Test category 2");
        fs::remove_file(temp_file).expect("Unable to remove file");
    }

    #[test]
    fn test_set_limit() {
        let temp_file = "test_data_7.json";
        let mut store = Store::new(Some(temp_file));
        store.set_limit(1000.0);
        assert_eq!(store.limit(), Some(1000.0));
        store.set_limit(0.0);
        assert_eq!(store.limit(), None);
        fs::remove_file(temp_file).expect("Unable to remove file");
    }

    #[test]
    fn test_get_category() {
        let temp_file = "test_data_8.json";
        let mut store = Store::new(Some(temp_file));
        let id = store.add_category("Test category");
        let category = store.get_category(id);
        assert_eq!(category.unwrap().name(), "Test category");
        fs::remove_file(temp_file).expect("Unable to remove file");
    }

    #[test]
    fn test_add_transaction_with_category() {
        let temp_file = "test_data_10.json";
        let mut store = Store::new(Some(temp_file));
        let category_id = store.add_category("Test category");
        let id = store.add_transaction("Test transaction".to_string(), 100.0, Some(store.get_category(category_id).unwrap()));
        assert_eq!(store.transactions.len(), 1);
        assert_eq!(store.transactions[0].id(), id);
        assert_eq!(store.transactions[0].description(), "Test transaction");
        assert_eq!(store.transactions[0].amount(), 100.0);
        assert_eq!(store.transactions[0].category().unwrap().name(), "Test category");
        fs::remove_file(temp_file).expect("Unable to remove file");
    }

    #[test]
    fn test_delete_category_with_transactions() {
        let temp_file = "test_data_11.json";
        let mut store = Store::new(Some(temp_file));
        let category_id = store.add_category("Test category");
        store.add_transaction("Test transaction".to_string(), 100.0, Some(store.get_category(category_id).unwrap()));
        store.delete_category(category_id);
        assert_eq!(store.categories.len(), 0);
        assert_eq!(store.transactions.len(), 1);
        assert_eq!(store.transactions[0].category(), None);
        fs::remove_file(temp_file).expect("Unable to remove file");
    }   
}