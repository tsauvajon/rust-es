use crate::cqrs::aggregate;

pub trait DomainEvent<T: aggregate::Aggregate> {
    fn apply(self, aggregate: &mut T);
}

pub enum BalanceEvent {
    PaymentMade(PaymentMade),
    ClearanceMade(ClearanceMade),
    RideFinished(RideFinished),
}

impl DomainEvent<aggregate::Balance> for BalanceEvent {
    fn apply(self, account: &mut aggregate::Balance) {
        match self {
            BalanceEvent::PaymentMade(e) => e.apply(account),
            BalanceEvent::ClearanceMade(e) => e.apply(account),
            BalanceEvent::RideFinished(e) => e.apply(account),
        }
    }
}

pub struct PaymentMade {
    pub amount: f64,
}

impl DomainEvent<aggregate::Balance> for PaymentMade {
    fn apply(self, balance: &mut aggregate::Balance) {
        balance.amount += self.amount
    }
}

pub struct ClearanceMade {
    pub amount: f64,
}

impl DomainEvent<aggregate::Balance> for ClearanceMade {
    fn apply(self, balance: &mut aggregate::Balance) {
        balance.amount -= self.amount
    }
}

pub struct RideFinished {
    pub amount: f64,
}

impl DomainEvent<aggregate::Balance> for RideFinished {
    fn apply(self, balance: &mut aggregate::Balance) {
        balance.amount += self.amount
    }
}
