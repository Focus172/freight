use std::fmt;

use crate::prelude::*;

pub trait YumaCallbackSig = FnOnce() -> Result<()> + 'static;
pub struct YumaCallback(Box<dyn YumaCallbackSig>);

impl fmt::Debug for YumaCallback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("YumaCallback").field(&"..").finish()
    }
}

impl YumaCallback {
    pub fn new<F>(f: F) -> Self
    where
        F: YumaCallbackSig,
    {
        YumaCallback(Box::new(f))
    }

    pub fn call(self) -> Result<()> {
        self.0()
    }
}

#[derive(Default, Debug)]
pub struct Callbacks {
    pub queued: Vec<(String, YumaCallback)>,
}

impl Callbacks {
    pub const fn new() -> Self {
        Self { queued: Vec::new() }
    }

    pub fn add<S, F>(&mut self, name: S, f: F)
    where
        S: Into<String>,
        F: YumaCallbackSig,
    {
        self.queued.push((name.into(), YumaCallback::new(f)))
    }
}
