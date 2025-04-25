use chrono::prelude::{DateTime, Utc};
use serde::{Serialize, Deserialize};

pub type TransactionId = u32;
pub type CategoryId = u32;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    id: TransactionId,
    amount: f64,
    description: String,
    datetime: DateTime<Utc>,
    category: Option<Category>
}

impl Transaction {
    pub fn new(id: TransactionId, amount: f64, description: String, category: Option<Category>) -> Self {
        Transaction {
            id,
            amount,
            description,
            datetime: Utc::now(),
            category
        }
    }

    pub fn id(&self) -> TransactionId {
        self.id
    }

    pub fn category(&self) -> Option<Category> {
        self.category.clone()
    }

    pub fn remove_category(&mut self) {
        self.category = None;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Category {
    id: CategoryId,
    name: String,
}

impl Category {
    pub fn new(id: CategoryId, name: String) -> Self {
        Category { id, name }
    }

    pub fn id(&self) -> CategoryId {
        self.id
    }
    
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Limit(f64);

impl Limit {
    pub fn new(amount: f64) -> Self {
        Limit(amount)
    }
}

