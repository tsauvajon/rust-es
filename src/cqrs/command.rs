use crate::cqrs::aggregate;
use crate::cqrs::event;

use uuid::Uuid;

pub trait Command<A: aggregate::Aggregate, E: event::DomainEvent<A>> {
    fn handle(self, aggregate: &A) -> Result<Vec<E>, aggregate::Error>;
}

pub struct MakePayment {
    driver_id: Uuid,
    amount: f64,
}

impl Command<aggregate::Balance, event::PaymentMade> for MakePayment {
    fn handle(
        self,
        _balance: &aggregate::Balance,
    ) -> Result<Vec<event::PaymentMade>, aggregate::Error> {
        println!(
            "make payment of {:?} for driver {:?}",
            self.amount, self.driver_id
        );
        Ok(vec![])
    }
}

pub struct MakeClearance {
    driver_id: Uuid,
    amount: f64,
}

impl Command<aggregate::Balance, event::ClearanceMade> for MakeClearance {
    fn handle(
        self,
        balance: &aggregate::Balance,
    ) -> Result<Vec<event::ClearanceMade>, aggregate::Error> {
        if balance.amount - self.amount < 0_f64 {
            return Err(aggregate::Error::UserError(
                "insufficient funds".to_string(),
            ));
        }

        println!(
            "make clearance of {:?} for driver {:?}",
            self.amount, self.driver_id
        );
        Ok(vec![event::ClearanceMade {
            amount: self.amount,
        }])
    }
}

pub struct FinishRide {
    driver_id: Uuid,
    passenger_id: String,
    amount: f64,
}
impl Command<aggregate::Balance, event::RideFinished> for FinishRide {
    fn handle(
        self,
        _balance: &aggregate::Balance,
    ) -> Result<Vec<event::RideFinished>, aggregate::Error> {
        println!(
            "make payment of {:?} for driver {:?} and passenger {:?}",
            self.amount, self.driver_id, self.passenger_id
        );
        Ok(vec![])
    }
}
