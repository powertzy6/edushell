// SPDX-License-Identifier: GPL-3.0-or-later

//! # Logging Framework
//!
//! Structured logging backed by the `tracing` crate.
//! Supports multiple output targets (stdout, file)
//! with configurable levels.

use std::path::PathBuf;

use tracing_subscriber::{
    filter, fmt, prelude::*, registry::Registry, util::SubscriberInitExt,
};

use crate::error::EduResult;

/// Log level configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for tracing::Level {
    fn from(l: LogLevel) -> Self {
        match l {
            LogLevel::Trace => tracing::Level::TRACE,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Error => tracing::Level::ERROR,
        }
    }
}

impl From<LogLevel> for tracing_subscriber::filter::LevelFilter {
    fn from(l: LogLevel) -> Self {
        match l {
            LogLevel::Trace => tracing_subscriber::filter::LevelFilter::TRACE,
            LogLevel::Debug => tracing_subscriber::filter::LevelFilter::DEBUG,
            LogLevel::Info => tracing_subscriber::filter::LevelFilter::INFO,
            LogLevel::Warn => tracing_subscriber::filter::LevelFilter::WARN,
            LogLevel::Error => tracing_subscriber::filter::LevelFilter::ERROR,
        }
    }
}

impl std::str::FromStr for LogLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "trace" => Ok(Self::Trace),
            "debug" => Ok(Self::Debug),
            "info" => Ok(Self::Info),
            "warn" => Ok(Self::Warn),
            "error" => Ok(Self::Error),
            _ => Err(format!("invalid log level: {s}")),
        }
    }
}

/// Configuration for the logging system.
#[derive(Debug, Clone)]
pub struct LogConfig {
    pub file_level: LogLevel,
    pub stdout_level: LogLevel,
    pub log_dir: Option<PathBuf>,
    pub max_file_size_bytes: u64,
    pub json_format: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            file_level: LogLevel::Info,
            stdout_level: if cfg!(debug_assertions) {
                LogLevel::Debug
            } else {
                LogLevel::Info
            },
            log_dir: None,
            max_file_size_bytes: 50 * 1024 * 1024,
            json_format: false,
        }
    }
}

/// Initialize the logging framework.
pub fn init_logging(config: &LogConfig) -> EduResult<()> {
    let log_dir = config
        .log_dir
        .clone()
        .unwrap_or_else(|| {
            dirs::data_dir()
                .unwrap_or_else(|| PathBuf::from("/tmp"))
                .join("edushell/logs")
        });

    std::fs::create_dir_all(&log_dir).map_err(crate::error::EduError::Io)?;

    let log_file_path = log_dir.join("edushell.log");

    let file_filter = filter::Targets::new()
        .with_target("edushell", config.file_level)
        .with_default(config.file_level);

    let stdout_filter = filter::Targets::new()
        .with_target("edushell", config.stdout_level)
        .with_default(config.stdout_level);

    let file_appender = RollingFileWriter::new(log_file_path, config.max_file_size_bytes);

    if config.json_format {
        let file_layer = fmt::layer()
            .json()
            .with_writer(file_appender)
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
            .with_filter(file_filter);

        let stdout_layer = fmt::layer()
            .with_writer(std::io::stdout)
            .with_ansi(true)
            .pretty()
            .with_filter(stdout_filter);

        Registry::default().with(file_layer).with(stdout_layer).init();
    } else {
        let file_layer = fmt::layer()
            .with_writer(file_appender)
            .with_target(true)
            .with_ansi(false)
            .with_filter(file_filter);

        let stdout_layer = fmt::layer()
            .with_writer(std::io::stdout)
            .with_ansi(true)
            .with_filter(stdout_filter);

        Registry::default().with(file_layer).with(stdout_layer).init();
    }

    tracing::info!(
        target: "edushell::logging",
        log_dir = %log_dir.display(),
        file_level = ?config.file_level,
        "Logging initialized"
    );

    Ok(())
}

/// A rolling file writer that wraps a Mutex<File>.
struct RollingFileWriter {
    path: PathBuf,
    max_size: u64,
}

impl RollingFileWriter {
    fn new(path: PathBuf, max_size: u64) -> Self {
        Self { path, max_size }
    }

    fn open(&self) -> std::io::Result<std::fs::File> {
        std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
    }

    fn rotate(&self) {
        if let Ok(meta) = std::fs::metadata(&self.path) {
            if meta.len() >= self.max_size {
                let rotated = self.path.with_extension("log.old");
                let _ = std::fs::rename(&self.path, &rotated);
            }
        }
    }
}

impl tracing_subscriber::fmt::MakeWriter<'_> for RollingFileWriter {
    type Writer = std::fs::File;

    fn make_writer(&self) -> Self::Writer {
        self.rotate();
        self.open().expect("Failed to open log file")
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_from_str() {
        assert_eq!("info".parse::<LogLevel>().unwrap(), LogLevel::Info);
        assert_eq!("DEBUG".parse::<LogLevel>().unwrap(), LogLevel::Debug);
        assert!("invalid".parse::<LogLevel>().is_err());
    }

    #[test]
    fn test_log_config_defaults() {
        let cfg = LogConfig::default();
        assert_eq!(cfg.file_level, LogLevel::Info);
        assert_eq!(cfg.max_file_size_bytes, 50 * 1024 * 1024);
    }
}
