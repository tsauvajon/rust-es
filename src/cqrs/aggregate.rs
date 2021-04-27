use uuid::Uuid;

pub trait Aggregate: Default {}

#[derive(Debug, PartialEq)]
pub enum Error {
    UserError(String),
    TechnicalError(String),
}

pub struct Balance {
    pub driver_id: Uuid,
    pub amount: f64,
}

impl Default for Balance {
    fn default() -> Self {
        Balance {
            amount: 0_f64,
            driver_id: Uuid::new_v4(),
        }
    }
}

impl Aggregate for Balance {}
