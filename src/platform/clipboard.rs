//! Cross-platform clipboard access.

use copypasta::{ClipboardContext, ClipboardProvider};

/// Easy wrapper for getting and setting text to the OS clipboard.
pub struct Clipboard {
    ctx: Result<ClipboardContext, String>,
}

impl Default for Clipboard {
    fn default() -> Self {
        Self::new()
    }
}

impl Clipboard {
    pub fn new() -> Self {
        Self {
            ctx: ClipboardContext::new().map_err(|e| format!("{:?}", e)), // copypasta errors are dynamic traits
        }
    }

    /// Read text from the clipboard. Returns `None` if it fails or is empty.
    pub fn get_text(&mut self) -> Option<String> {
        match &mut self.ctx {
            Ok(ctx) => {
                let text: Option<String> = ctx.get_contents().ok();
                text
            }
            Err(e) => {
                log::warn!("Clipboard not initialized: {}", e);
                None
            }
        }
    }

    /// Set text to the clipboard.
    pub fn set_text(&mut self, text: String) -> bool {
        match &mut self.ctx {
            Ok(ctx) => {
                let res: Result<(), _> = ctx.set_contents(text);
                res.is_ok()
            }
            Err(e) => {
                log::warn!("Clipboard not initialized: {}", e);
                false
            }
        }
    }
}
