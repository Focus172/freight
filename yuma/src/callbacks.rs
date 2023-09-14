use crate::prelude::YumaResult;

#[derive(Default)]
pub struct Callbacks {
    pub queued: Vec<YumaCallback>,
}

// type YumaCallback = Box<dyn FnOnce(Mutex<YumaCtx>) -> YumaResult>;
type YumaCallback = Box<dyn FnOnce() -> YumaResult>;

impl Callbacks {
    pub const fn new() -> Self {
        Self { queued: Vec::new() }
    }
}
