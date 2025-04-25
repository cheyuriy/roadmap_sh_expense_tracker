use super::models::{Transaction, Category, TransactionId, CategoryId};
use serde::{Serialize, Deserialize};
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Store {
    transactions: Vec<Transaction>,

    #[serde(skip)]
    max_transaction_id: TransactionId,

    categories: Vec<Category>,

    #[serde(skip)]
    max_category_id: CategoryId,

    limit: Option<f64>,

    #[serde(skip)]
    path: String
}

impl Store {
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

    fn persist(&self) {
        let json = serde_json::to_string_pretty(&self).expect("Unable to write JSON");

        let path = Path::new(&self.path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("Unable to create directory");
        }
        let mut file = fs::File::create(&self.path).expect("Unable to create file");
        file.write_all(json.as_bytes()).expect("Unable to write file");
    }

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

    pub fn delete_transaction(&mut self, id: TransactionId) {
        if let Some(pos) = self.transactions.iter().position(|transaction| transaction.id() == id) {
            self.transactions.remove(pos);
            self.persist();
        }
    }

    pub fn list_transactions(&self, category: Option<Category>) -> Vec<&Transaction> {
        if let Some(_) = category {
            self.transactions.iter().filter(|&transaction| transaction.category() == category).collect()
        } else {
            self.transactions.iter().collect()
        }
    }

    pub fn get_category(&self, id: CategoryId) -> Option<Category> {
        if let Some(cat) = self.categories.iter().find(|&cat| cat.id() == id) {
            Some(cat.clone())
        } else {
            None    
        }
    }

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

    pub fn list_categories(&self) -> &Vec<Category> {
        &self.categories
    }

    
    
}
