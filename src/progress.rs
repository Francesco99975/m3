use std::time::Duration;

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

pub struct CliLoading {
    bar: ProgressBar,
}

impl CliLoading {
    pub fn new() -> Self {
        let loading = Self {
            bar: ProgressBar::new_spinner(),
        };
        loading.bar.enable_steady_tick(Duration::from_millis(120));
        loading.bar.set_style(
            ProgressStyle::with_template("{spinner:.blue} {msg}")
                .unwrap()
                .tick_strings(&[
                    "▹▹▹▹▹",
                    "▸▹▹▹▹",
                    "▹▸▹▹▹",
                    "▹▹▸▹▹",
                    "▹▹▹▸▹",
                    "▹▹▹▹▸",
                    "▪▪▪▪▪",
                ]),
        );
        loading
    }

    pub fn set(&self, message: &str) {
        self.bar.set_message(format!("{}", message.blue().italic()));
    }

    pub fn end(&self, message: &str) {
        self.bar
            .finish_with_message(format!("{}", message.blue().italic()));
    }
}
