use uuid::Uuid;

pub trait Aggregate: Default {}

#[derive(Debug, PartialEq)]
pub enum Error {
    UserError(String),
    TechnicalError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
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

#[cfg(test)]
mod aggregate_tests {
    use super::*;
    use crate::cqrs::testing::TestFramework;
    use crate::cqrs::{command, event};

    type BalanceTests = TestFramework<Balance, event::BalanceEvent>;

    #[test]
    fn test_make_payment() {
        let id = Uuid::new_v4();

        let expected = event::BalanceEvent::DriverMadePayment(event::DriverMadePayment {
            amount: 12.34_f64,
            driver_id: id,
            balance: 12.34_f64,
        });

        BalanceTests::default()
            .given_no_previous_events()
            .when(command::MakePayment {
                amount: 12.34,
                driver_id: id,
            })
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_make_payment_with_balance() {
        let id = Uuid::new_v4();

        let previous = event::BalanceEvent::DriverMadePayment(event::DriverMadePayment { driver_id:id, amount: 100.0, balance: 100.0 });
        let expected = event::BalanceEvent::DriverMadePayment(event::DriverMadePayment { driver_id:id, amount: 200.0, balance: 300.0 });

        BalanceTests::default()
            .given(vec![previous])
            .when(command::MakePayment{ amount: 200.0, driver_id: id })
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_make_clearance() {
        let id = Uuid::new_v4();

        let previous = event::BalanceEvent::DriverMadePayment(event::DriverMadePayment { driver_id:id, amount: 100.0, balance: 100.0 });

        let expected = event::BalanceEvent::ClearanceSentToDriver(event::ClearanceSentToDriver {
            amount: 40_f64,
            driver_id: id,
            balance: 60_f64,
        });

        BalanceTests::default()
            .given(vec![previous])
            .when(command::MakeClearance {
                amount: 40_f64,
                driver_id: id,
            })
            .then_expect_events(vec![expected]);
    }

    #[test]
    fn test_make_clearance_insufficient_balance() {
        let id = Uuid::new_v4();

        let previous = event::BalanceEvent::DriverMadePayment(event::DriverMadePayment { driver_id:id, amount: 100.0, balance: 100.0 });

        BalanceTests::default()
            .given(vec![previous])
            .when(command::MakeClearance {
                amount: 400_f64,
                driver_id: id,
            })
            .then_expect_error("insufficient funds")
    }
}
