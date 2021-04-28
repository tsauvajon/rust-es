use crate::cqrs::aggregate;
use crate::cqrs::event;

use uuid::Uuid;

pub trait Command<A: aggregate::Aggregate, E: event::DomainEvent<A>> {
    fn handle(self, aggregate: &A) -> Result<Vec<E>, aggregate::Error>;
}

pub struct MakePayment {
    pub driver_id: Uuid,
    pub amount: f64,
}

impl Command<aggregate::Balance, event::BalanceEvent> for MakePayment {
    fn handle(
        self,
        balance: &aggregate::Balance,
    ) -> Result<Vec<event::BalanceEvent>, aggregate::Error> {
        println!(
            "make payment of {:?} for driver {:?}",
            self.amount, self.driver_id
        );
        
        Ok(vec![event::BalanceEvent::DriverMadePayment(
            event::DriverMadePayment {
                amount: self.amount,
                driver_id: self.driver_id,
                balance: balance.amount + self.amount,
            },
        )])
    }
}

pub struct MakeClearance {
    pub driver_id: Uuid,
    pub amount: f64,
}

impl Command<aggregate::Balance, event::BalanceEvent> for MakeClearance {
    fn handle(
        self,
        balance: &aggregate::Balance,
    ) -> Result<Vec<event::BalanceEvent>, aggregate::Error> {
        if balance.amount - self.amount < 0_f64 {
            return Err(aggregate::Error::UserError(
                "insufficient funds".to_string(),
            ));
        }

        println!(
            "make clearance of {:?} for driver {:?}",
            self.amount, self.driver_id
        );

        Ok(vec![event::BalanceEvent::ClearanceSentToDriver(
            event::ClearanceSentToDriver {
                amount: self.amount,
                driver_id: self.driver_id,
                balance: balance.amount - self.amount,
            },
        )])
    }
}

pub struct FinishRide {
    pub driver_id: Uuid,
    pub passenger_id: Uuid,
    pub fare: f64,
}
impl Command<aggregate::Balance, event::BalanceEvent> for FinishRide {
    fn handle(
        self,
        balance: &aggregate::Balance,
    ) -> Result<Vec<event::BalanceEvent>, aggregate::Error> {
        println!(
            "ride with fare {:?} finished for driver {:?} and passenger {:?}",
            self.fare, self.driver_id, self.passenger_id
        );
        
        
        Ok(vec![event::BalanceEvent::RideFinished(
            event::RideFinished {
                fare: self.fare,
                driver_id: self.driver_id,
                passenger_id: self.passenger_id,
                driver_balance: balance.amount + self.fare,
            },
        )])
    }
}
