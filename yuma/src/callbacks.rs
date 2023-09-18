use crate::prelude::YumaResult;

#[derive(Default)]
pub struct Callbacks {
    pub queued: Vec<(String, YumaCallback)>,
}

// type YumaCallback = Box<dyn FnOnce(Mutex<YumaCtx>) -> YumaResult>;
type YumaCallback = Box<dyn YumaCallbackSig>;
pub trait YumaCallbackSig = FnOnce() -> YumaResult + 'static;

impl Callbacks {
    pub const fn new() -> Self {
        Self { queued: Vec::new() }
    }

    pub fn add(&mut self, name: impl Into<String>, f: impl YumaCallbackSig) {
        self.queued.push((name.into(), Box::new(f)))
    }
}
