// Turns one line of text into a LogEntry.
//
// Expected format (one entry per line):
//   2026-10-24T10:15:32 INFO  [auth] User asha logged in (12ms)

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

impl Level {
    pub fn parse(text: &str) -> Option<Level> {
        match text.to_ascii_uppercase().as_str() {
            "DEBUG" => Some(Level::Debug),
            "INFO" => Some(Level::Info),
            "WARN" | "WARNING" => Some(Level::Warn),
            "ERROR" => Some(Level::Error),
            _ => None,
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
        };
        f.pad(name) // pad() respects width options like {:<5}
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: Level,
    pub module: String,
    pub message: String,
    pub duration_ms: Option<u32>,
}

/// Parses a line, or explains why it could not.
pub fn parse_line(line: &str) -> Result<LogEntry, String> {
    let mut parts = line.splitn(3, ' ');
    let timestamp = parts.next().filter(|t| t.contains('T')).ok_or("missing timestamp")?;
    let level_text = parts.next().ok_or("missing level")?;
    let level = Level::parse(level_text).ok_or(format!("unknown level '{level_text}'"))?;
    let rest = parts.next().ok_or("missing module")?.trim_start();

    let rest = rest.strip_prefix('[').ok_or("module must be in [brackets]")?;
    let (module, message) = rest.split_once("] ").ok_or("module must be in [brackets]")?;

    // An optional "(123ms)" at the end of the message.
    let (message, duration_ms) = match message.rsplit_once(" (") {
        Some((msg, tail)) if tail.ends_with("ms)") => {
            let number = &tail[..tail.len() - 3];
            (msg, number.parse().ok())
        }
        _ => (message, None),
    };

    Ok(LogEntry {
        timestamp: timestamp.to_string(),
        level,
        module: module.to_string(),
        message: message.to_string(),
        duration_ms,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_full_line() {
        let e = parse_line("2026-10-24T10:15:32 INFO  [auth] User asha logged in (12ms)").unwrap();
        assert_eq!(e.level, Level::Info);
        assert_eq!(e.module, "auth");
        assert_eq!(e.message, "User asha logged in");
        assert_eq!(e.duration_ms, Some(12));
    }

    #[test]
    fn duration_is_optional() {
        let e = parse_line("2026-10-24T10:15:32 WARN [db] Disk 91% full").unwrap();
        assert_eq!(e.duration_ms, None);
        assert_eq!(e.message, "Disk 91% full");
    }

    #[test]
    fn explains_bad_lines() {
        assert_eq!(parse_line("hello world"), Err("missing timestamp".to_string()));
        assert!(parse_line("2026-10-24T10:15:32 LOUD [x] y").unwrap_err().contains("unknown level"));
        assert!(parse_line("2026-10-24T10:15:32 INFO no brackets").is_err());
    }
}
