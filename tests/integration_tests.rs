use okerrr::okerrr;
use std::cell::Cell;
use std::future::Future;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

struct NoopWake;

impl Wake for NoopWake {
    fn wake(self: Arc<Self>) {}
}

fn poll_ready<F: Future>(future: F) -> F::Output {
    let waker = Waker::from(Arc::new(NoopWake));
    let mut context = Context::from_waker(&waker);
    let mut future = Box::pin(future);

    match future.as_mut().poll(&mut context) {
        Poll::Ready(output) => output,
        Poll::Pending => panic!("test future unexpectedly returned pending"),
    }
}

#[test]
fn extracts_ok_payload() {
    fn double(input: Result<i32, &'static str>) -> Result<i32, &'static str> {
        let value = okerrr!(input, case error => return Err(error));
        Ok(value * 2)
    }

    assert_eq!(double(Ok(21)), Ok(42));
}

#[test]
fn binds_and_transforms_err_payload() {
    fn double(input: Result<i32, &'static str>) -> Result<i32, String> {
        let value = okerrr!(input, case error => {
            return Err(format!("invalid input: {error}"));
        });
        Ok(value * 2)
    }

    assert_eq!(double(Err("empty")), Err("invalid input: empty".into()));
}

#[test]
fn evaluates_input_once() {
    fn extract(calls: &Cell<usize>, input: Result<i32, &'static str>) -> Result<i32, &'static str> {
        let value = okerrr!(
            {
                calls.set(calls.get() + 1);
                input
            },
            case error => return Err(error)
        );
        Ok(value)
    }

    let calls = Cell::new(0);
    assert_eq!(extract(&calls, Ok(7)), Ok(7));
    assert_eq!(calls.get(), 1);

    assert_eq!(extract(&calls, Err("bad")), Err("bad"));
    assert_eq!(calls.get(), 2);
}

#[test]
fn controls_enclosing_loop() {
    enum ItemError {
        Ignore,
        Skip,
        Stop,
    }

    let items = [
        Ok(1),
        Err(ItemError::Ignore),
        Err(ItemError::Skip),
        Ok(3),
        Err(ItemError::Stop),
        Ok(5),
    ];
    let mut sum = 0;

    for item in items {
        let value = okerrr!(
            item,
            case ItemError::Ignore | ItemError::Skip => continue,
            case ItemError::Stop => break,
        );
        sum += value;
    }

    assert_eq!(sum, 4);
}

#[test]
fn accepts_explicit_panic_without_formatting_error() {
    struct Opaque;

    let panic = std::panic::catch_unwind(|| {
        let input: Result<i32, Opaque> = Err(Opaque);
        okerrr!(input, case _ => panic!("required value was missing"))
    });

    assert!(panic.is_err());
}

#[test]
fn accepts_awaited_input_in_async_context() {
    async fn load(input: Result<i32, &'static str>) -> Result<i32, &'static str> {
        let value = okerrr!(async { input }.await, case error => return Err(error));
        Ok(value)
    }

    assert_eq!(poll_ready(load(Ok(7))), Ok(7));
    assert_eq!(poll_ready(load(Err("unavailable"))), Err("unavailable"));
}

#[test]
fn borrows_shared_payload_without_consuming_result() {
    fn extract(input: &Result<String, String>) -> Result<&str, &str> {
        let value = okerrr!(input, case error => return Err(error.as_str()));
        Ok(value.as_str())
    }

    let input = Ok(String::from("value"));

    assert_eq!(extract(&input), Ok("value"));
    assert_eq!(input.as_deref(), Ok("value"));
}

#[test]
fn borrows_mutable_payload_from_either_variant() {
    fn increment(input: &mut Result<i32, i32>) {
        let value = okerrr!(input, case error => {
            *error += 1;
            return;
        });
        *value += 1;
    }

    let mut success = Ok(1);
    increment(&mut success);
    assert_eq!(success, Ok(2));

    let mut failure = Err(1);
    increment(&mut failure);
    assert_eq!(failure, Err(2));
}

#[test]
fn dispatches_destructured_errors_with_guards() {
    #[derive(Debug, PartialEq)]
    enum FetchError {
        Busy { attempt: u8 },
        Fatal,
    }

    fn load(input: Result<i32, FetchError>, retries: u8) -> Result<i32, FetchError> {
        let value = okerrr!(
            input,
            case FetchError::Busy { attempt } if attempt < retries => return Ok(attempt.into()),
            case error @ FetchError::Busy { .. } => return Err(error),
            case error => return Err(error),
        );
        Ok(value)
    }

    assert_eq!(load(Ok(7), 3), Ok(7));
    assert_eq!(load(Err(FetchError::Busy { attempt: 2 }), 3), Ok(2));
    assert_eq!(
        load(Err(FetchError::Busy { attempt: 3 }), 3),
        Err(FetchError::Busy { attempt: 3 })
    );
    assert_eq!(load(Err(FetchError::Fatal), 3), Err(FetchError::Fatal));
}
