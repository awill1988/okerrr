use okerrr::okerrr;
use std::cell::Cell;

#[test]
fn extracts_ok_payload() {
    fn double(input: Result<i32, &'static str>) -> Result<i32, &'static str> {
        let value = okerrr!(input, else error => return Err(error));
        Ok(value * 2)
    }

    assert_eq!(double(Ok(21)), Ok(42));
}

#[test]
fn binds_and_transforms_err_payload() {
    fn double(input: Result<i32, &'static str>) -> Result<i32, String> {
        let value = okerrr!(input, else error => {
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
            else error => return Err(error)
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
    let items = [Ok(1), Err("skip"), Ok(3), Err("stop"), Ok(5)];
    let mut sum = 0;

    for item in items {
        let value = okerrr!(item, else error => {
            if error == "stop" {
                break;
            }
            continue;
        });
        sum += value;
    }

    assert_eq!(sum, 4);
}

#[test]
fn accepts_explicit_panic_without_formatting_error() {
    struct Opaque;

    let panic = std::panic::catch_unwind(|| {
        let input: Result<i32, Opaque> = Err(Opaque);
        okerrr!(input, else _error => panic!("required value was missing"))
    });

    assert!(panic.is_err());
}

#[test]
fn accepts_awaited_input_in_async_context() {
    async fn load(input: Result<i32, &'static str>) -> Result<i32, &'static str> {
        let value = okerrr!(async { input }.await, else error => return Err(error));
        Ok(value)
    }

    let future = load(Ok(7));
    drop(future);
}
