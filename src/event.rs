use crate::aggregate;

use uuid::Uuid;

pub trait DomainEvent<T: aggregate::Aggregate>: std::cmp::PartialEq + std::fmt::Debug {
    fn apply(self, aggregate: &mut T);
}

#[derive(Debug, PartialEq)]
pub enum BalanceEvent {
    DriverMadePayment(DriverMadePayment),
    ClearanceSentToDriver(ClearanceSentToDriver),
    RideFinished(RideFinished),
}

impl DomainEvent<aggregate::Balance> for BalanceEvent {
    fn apply(self, account: &mut aggregate::Balance) {
        match self {
            BalanceEvent::DriverMadePayment(e) => e.apply(account),
            BalanceEvent::ClearanceSentToDriver(e) => e.apply(account),
            BalanceEvent::RideFinished(e) => e.apply(account),
        }
    }
}

#[test]
fn balance_event_inequality() {
    let id = Uuid::new_v4();

    let a = BalanceEvent::ClearanceSentToDriver(ClearanceSentToDriver {
        amount: 10_f64,
        driver_id: id,
        balance: 123_f64,
    });
    let b = BalanceEvent::DriverMadePayment(DriverMadePayment {
        amount: 10_f64,
        driver_id: id,
        balance: 123_f64,
    });
    assert_eq!(false, a == b)
}

#[test]
fn balance_event_equality() {
    let driver_id = Uuid::new_v4();
    let passenger_id = Uuid::new_v4();

    let a = BalanceEvent::RideFinished(RideFinished {
        fare: 10_f64,
        driver_id: driver_id,
        passenger_id: passenger_id,
        driver_balance: 12.34,
    });
    let b = BalanceEvent::RideFinished(RideFinished {
        fare: 10_f64,
        driver_id: driver_id,
        passenger_id: passenger_id,
        driver_balance: 12.34,
    });
    assert_eq!(a, b)
}

#[derive(Debug, PartialEq)]
pub struct DriverMadePayment {
    pub driver_id: Uuid,
    pub amount: f64,
    pub balance: f64,
}

impl DomainEvent<aggregate::Balance> for DriverMadePayment {
    fn apply(self, balance: &mut aggregate::Balance) {
        balance.amount = self.balance
    }
}

#[derive(Debug, PartialEq)]
pub struct ClearanceSentToDriver {
    pub driver_id: Uuid,
    pub amount: f64,
    pub balance: f64,
}

impl DomainEvent<aggregate::Balance> for ClearanceSentToDriver {
    fn apply(self, balance: &mut aggregate::Balance) {
        balance.amount = self.balance
    }
}

#[derive(Debug, PartialEq)]
pub struct RideFinished {
    pub driver_id: Uuid,
    pub passenger_id: Uuid,
    pub fare: f64,
    pub driver_balance: f64,
}

impl DomainEvent<aggregate::Balance> for RideFinished {
    fn apply(self, balance: &mut aggregate::Balance) {
        balance.amount = self.driver_balance
    }
}
