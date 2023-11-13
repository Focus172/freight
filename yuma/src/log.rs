use crate::prelude::*;

pub use simplelog::{debug, error, info, trace, warn};

pub fn init() -> Result<()> {
    Logger::default().init()
}

#[derive(Debug)]
pub struct Logger {
    level: log::LevelFilter,
    config: simplelog::Config,
    mode: simplelog::TerminalMode,
    color_choice: simplelog::ColorChoice,
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            level: log::LevelFilter::Debug,
            config: simplelog::Config::default(),
            mode: simplelog::TerminalMode::Stderr,
            color_choice: simplelog::ColorChoice::Auto,
        }
    }
}

impl Logger {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn level(mut self, level: log::LevelFilter) -> Self {
        self.level = level;
        self
    }

    pub fn color(mut self, color: bool) {
        if color {
            self.color_choice = simplelog::ColorChoice::Always;
        } else {
            self.color_choice = simplelog::ColorChoice::Never;
        }
    }

    pub fn init(self) -> Result<()> {
        simplelog::TermLogger::init(self.level, self.config, self.mode, self.color_choice)?;
        Ok(())
    }
}
