use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Enums
#[derive(Serialize, Deserialize, Debug)]
pub enum TransactionType {
    EARNING,
    EXPENSE,
    INVESTIMENT,
}

// Structs
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub date: DateTime<Utc>,
    pub amount: f64,
    pub transaction_type: TransactionType,
}
