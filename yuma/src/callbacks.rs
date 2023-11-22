use std::fmt;

use stub::Stub;

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

type CallbackName = String;

#[derive(Default, Debug, Stub)]
pub struct Callbacks {
    pub queued: Vec<(CallbackName, YumaCallback)>,
    pub handles: Vec<(CallbackName, usize)>,
}

impl Callbacks {
    pub const fn new() -> Self {
        Self {
            queued: Vec::new(),
            handles: Vec::new(),
        }
    }

    pub fn add<F>(&mut self, name: String, f: F)
    where
        F: YumaCallbackSig,
    {
        self.queued.push((name, YumaCallback::new(f)))
    }

    pub fn run(&mut self) -> Result<()> {
        for (name, fun) in self.queued.drain(..) {
            crate::log::info!("<red>Running callback</>: {name}");
            fun.call()?;
        }
        Ok(())
    }

    // TODO: make the things run in the background and wait for them here
    pub fn wait(&mut self) -> Result<()> {
        Ok(())
    }

    // TODO: terminate all remaining callbacks
    pub fn abort(&mut self) {}
}

impl Drop for Callbacks {
    fn drop(&mut self) {
        self.abort();
    }
}
