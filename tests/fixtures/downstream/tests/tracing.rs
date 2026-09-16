#![cfg(feature = "with_tracing")]

use std::io::Write;
use std::sync::{Arc, Mutex};

use okerrr_downstream::{caller_traced, ordinary, Opaque};

struct Capture(Arc<Mutex<Vec<u8>>>);

impl Write for Capture {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(bytes);
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn caller_decides_when_to_trace() {
    let output = Arc::new(Mutex::new(Vec::new()));
    let writer = Arc::clone(&output);
    let subscriber = tracing_subscriber::fmt()
        .with_ansi(false)
        .without_time()
        .with_writer(move || Capture(Arc::clone(&writer)))
        .finish();

    tracing::subscriber::with_default(subscriber, || {
        assert_eq!(ordinary(Err(Opaque)), 0);
        assert!(output.lock().unwrap().is_empty());
        assert_eq!(caller_traced(Err("bad")), 0);
    });

    let message = String::from_utf8(output.lock().unwrap().clone()).unwrap();
    assert!(message.contains("caller handled error"));
    assert!(message.contains("bad"));
}
