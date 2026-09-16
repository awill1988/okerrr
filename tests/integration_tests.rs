use okerrr::{okerr, okerr_some, okerrr, okerrr_some, okurr, okurr_some, okurrr, okurrr_some};

#[test]
fn test_okerrr_bind_error_and_return() {
    fn inner(res: Result<i32, &'static str>) -> Result<i32, String> {
        let val = okerrr!(res, err => return Err(format!("bound: {err}")));
        Ok(val * 10)
    }

    assert_eq!(inner(Ok(5)), Ok(50));
    assert_eq!(inner(Err("bad")), Err("bound: bad".to_string()));
}

#[test]
fn test_okerrr_else_err_keyword() {
    fn inner(res: Result<i32, &'static str>) -> Result<i32, String> {
        let val = okerrr!(res, else err => return Err(format!("else_err: {err}")));
        Ok(val + 1)
    }

    assert_eq!(inner(Ok(99)), Ok(100));
    assert_eq!(inner(Err("fail")), Err("else_err: fail".to_string()));
}

#[test]
fn test_okerrr_else_fallback() {
    let ok_res: Result<i32, &'static str> = Ok(10);
    let err_res: Result<i32, &'static str> = Err("oops");

    let v1 = okerrr!(ok_res, else 0);
    let v2 = okerrr!(err_res, else -1);

    assert_eq!(v1, 10);
    assert_eq!(v2, -1);
}

#[test]
fn test_okerrr_shorthand_fallback() {
    let ok_res: Result<i32, &'static str> = Ok(20);
    let err_res: Result<i32, &'static str> = Err("oops");

    let v1 = okerrr!(ok_res, 0);
    let v2 = okerrr!(err_res, -1);

    assert_eq!(v1, 20);
    assert_eq!(v2, -1);
}

#[test]
fn test_okerrr_shorthand_early_return() {
    fn inner(res: Result<i32, &'static str>) -> Result<i32, String> {
        let val = okerrr!(res);
        Ok(val + 100)
    }

    assert_eq!(inner(Ok(5)), Ok(105));
    assert_eq!(inner(Err("error_msg")), Err("error_msg".to_string()));
}

#[test]
fn test_okerrr_in_loop_control_flow() {
    let items = vec![Ok(1), Err("skip"), Ok(3), Err("stop"), Ok(5)];
    let mut sum = 0;

    for item in items {
        let val = okerrr!(item, err => {
            if err == "stop" {
                break;
            } else {
                continue;
            }
        });
        sum += val;
    }

    assert_eq!(sum, 4); // 1 + 3
}

#[test]
fn test_okerr_aliases() {
    let res: Result<&str, &str> = Ok("OKERRR!");
    assert_eq!(okerr!(res, else "default"), "OKERRR!");
    assert_eq!(okurrr!(res, else "default"), "OKERRR!");
    assert_eq!(okurr!(res, else "default"), "OKERRR!");
}

#[test]
fn test_okerrr_some_and_aliases() {
    fn find_val(opt: Option<i32>) -> Option<i32> {
        let v = okerrr_some!(opt);
        Some(v * 2)
    }

    assert_eq!(find_val(Some(21)), Some(42));
    assert_eq!(find_val(None), None);

    let opt_none: Option<i32> = None;
    assert_eq!(okerrr_some!(opt_none, else 100), 100);
    assert_eq!(okerr_some!(opt_none, else 100), 100);
    assert_eq!(okurrr_some!(opt_none, else 100), 100);
    assert_eq!(okurr_some!(opt_none, 200), 200);
}

