//! Panic handler that displays a fallback error screen instead of crashing silently.

use std::panic::PanicHookInfo;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use log::error;

static PANICKED: AtomicBool = AtomicBool::new(false);

lazy_static::lazy_static! {
    pub static ref CRASH_REPORT: Mutex<String> = Mutex::new(String::new());
}

/// Sets the global panic hook to capture panic information
/// and allow the Engine to safely shut down or transition to an error state.
pub fn init_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info: &PanicHookInfo| {
        PANICKED.store(true, Ordering::SeqCst);
        
        // Format the panic message.
        let msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            *s
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.as_str()
        } else {
            "Unknown panic payload."
        };

        // Extract location
        let location = if let Some(loc) = panic_info.location() {
            format!("{}:{}:{}", loc.file(), loc.line(), loc.column())
        } else {
            "Unknown location".to_string()
        };

        let report = format!("Veltrix Engine Panic!\n\nLocation: {}\nMessage: {}", location, msg);
        error!("{}", report);
        
        if let Ok(mut lock) = CRASH_REPORT.lock() {
            *lock = report;
        }
    }));
}

/// Returns true if a panic has been caught by the custom hook.
pub fn has_panicked() -> bool {
    PANICKED.load(Ordering::SeqCst)
}

/// Retrieves the formatted crash report string.
pub fn get_crash_report() -> String {
    if let Ok(lock) = CRASH_REPORT.lock() {
        lock.to_string()
    } else {
        "Veltrix Engine Panic (Mutex poisoned)".to_string()
    }
}
