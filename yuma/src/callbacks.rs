use crate::{prelude::YumaResult, YumaCtx};

#[derive(Default)]
pub struct Callbacks {
    pub queued: Vec<YumaCallback>,
}

// type YumaCallback = Box<dyn FnOnce(Mutex<YumaCtx>) -> YumaResult>;
type YumaCallback = Box<dyn FnOnce(Box<dyn AsMut<YumaCtx>>) -> YumaResult>;

impl Callbacks {
    pub const fn new() -> Self {
        Self { queued: Vec::new() }
    }
}
