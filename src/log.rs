use colored::Colorize;
use notify_rust::{Notification, Timeout};
use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex, PoisonError},
    time::{Duration, Instant},
};

static LAST_LOGS: LazyLock<Mutex<HashMap<String, Instant>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

const LOG_COOLDOWN: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Log {
    Info,
    Error,
}

impl Log {
    pub fn log(self, message: &str) {
        {
            let mut logs = LAST_LOGS.lock().unwrap_or_else(PoisonError::into_inner);

            let now = Instant::now();

            logs.retain(|_, t| now.duration_since(*t) < Duration::from_mins(1));

            match logs.get(message) {
                Some(last) if now.duration_since(*last) < LOG_COOLDOWN => return,
                _ => {
                    logs.insert(message.to_owned(), now);
                }
            }
        }

        let mut notification = Notification::new();
        notification
            .summary("ProjMan:")
            .body(message)
            .timeout(Timeout::Milliseconds(4000));

        match self {
            Self::Error => {
                let _ = notification.icon("dialog-error").show();

                eprintln!("{} {}", "[ERROR]:".red().bold(), message.bold());
            }
            Self::Info => {
                let _ = notification.icon("dialog-information").show();

                println!("{} {}", "[INFO]:".blue().bold(), message.bold());
            }
        }
    }
}
