use crate::cqrs::{aggregate, command, event};
use std::marker::PhantomData;

pub struct TestFramework<A, E> {
    _phantom: PhantomData<(A, E)>,
}

impl<A, E> TestFramework<A, E>
where
    A: aggregate::Aggregate,
    E: event::DomainEvent<A>,
{
    /// Initiates an aggregate test with no previous events.
    #[must_use]
    pub fn given_no_previous_events(&self) -> AggregateTestExecutor<A, E> {
        AggregateTestExecutor {
            events: Vec::new(),
            _phantom: PhantomData,
        }
    }
    /// Initiates an aggregate test with a collection of previous events.
    #[must_use]
    pub fn given(&self, events: Vec<E>) -> AggregateTestExecutor<A, E> {
        AggregateTestExecutor {
            events,
            _phantom: PhantomData,
        }
    }
}

impl<A, E> Default for TestFramework<A, E>
where
    A: aggregate::Aggregate,
    E: event::DomainEvent<A>,
{
    fn default() -> Self {
        TestFramework {
            _phantom: PhantomData,
        }
    }
}

/// Holds the initial event state of an aggregate and accepts a command.
pub struct AggregateTestExecutor<A, E>
where
    A: aggregate::Aggregate,
    E: event::DomainEvent<A>,
{
    events: Vec<E>,
    _phantom: PhantomData<A>,
}

impl<A, E> AggregateTestExecutor<A, E>
where
    A: aggregate::Aggregate,
    E: event::DomainEvent<A>,
{
    /// Consumes a command and using the state details previously passed provides a validator object
    /// to test against.
    pub fn when<C: command::Command<A, E>>(self, command: C) -> AggregateResultValidator<A, E> {
        let mut aggregate = A::default();
        for event in self.events {
            event.apply(&mut aggregate)
        }
        let result = command.handle(&aggregate);
        AggregateResultValidator {
            result,
            _phantom: PhantomData,
        }
    }
}

/// Validation object for the `TestFramework` package.
pub struct AggregateResultValidator<A, E>
where
    A: aggregate::Aggregate,
    E: event::DomainEvent<A>,
{
    result: Result<Vec<E>, aggregate::Error>,
    _phantom: PhantomData<A>,
}

impl<A, E> AggregateResultValidator<A, E>
where
    A: aggregate::Aggregate,
    E: event::DomainEvent<A>,
{
    /// Verifies that the expected events have been produced by the command.
    pub fn then_expect_events(self, expected_events: Vec<E>) {
        let events = match self.result {
            Ok(expected_events) => expected_events,
            Err(err) => {
                panic!("expected success, received aggregate error: '{}'", err);
            }
        };
        assert_eq!(&events[..], &expected_events[..]);
    }
    /// Verifies that an `aggregate::Error` with the expected message is produced with the command.
    pub fn then_expect_error(self, error_message: &str) {
        match self.result {
            Ok(events) => {
                panic!("expected error, received events: '{:?}'", events);
            }
            Err(err) => match err {
                aggregate::Error::TechnicalError(err) => {
                    panic!("expected user error but found technical error: {}", err)
                }
                aggregate::Error::UserError(err) => {
                    assert_eq!(err, error_message.to_string());
                }
            },
        };
    }
}
