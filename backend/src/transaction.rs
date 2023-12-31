use chrono::NaiveDate;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Transaction {
    pub source: Source,
    pub date: NaiveDate,
    pub amount: f32,
    pub description: String,
    pub skip: bool,
}
impl Transaction {
    pub fn new(source: Source, date: NaiveDate, amount: f32, description: String) -> Transaction {
        Transaction {
            source: source,
            date: date,
            amount: amount,
            description: description,
            skip: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize)]
pub enum Source {
    Chase,
    ChaseBank,
    BofA,
    Amex,
    Venmo,
    Bilt,
}
