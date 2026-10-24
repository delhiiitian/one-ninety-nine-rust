// Collects statistics while entries stream past, then renders a report.
// Nothing here touches files: it works on any source of entries.

use crate::parser::{Level, LogEntry};
use std::collections::{BTreeMap, HashMap};
use std::fmt::Write;

#[derive(Default)]
pub struct Stats {
    pub total_lines: usize,
    pub parsed: usize,
    pub bad_lines: Vec<(usize, String)>, // (line number, reason)
    pub by_level: BTreeMap<Level, usize>,
    pub by_module: HashMap<String, usize>,
    pub errors: Vec<LogEntry>,
    pub slowest: Vec<LogEntry>, // kept sorted, longest first
    pub first_time: Option<String>,
    pub last_time: Option<String>,
    total_ms: u64,
    timed: usize,
}

const KEEP_SLOWEST: usize = 5;

impl Stats {
    pub fn add(&mut self, entry: LogEntry) {
        self.parsed += 1;
        *self.by_level.entry(entry.level).or_insert(0) += 1;
        *self.by_module.entry(entry.module.clone()).or_insert(0) += 1;
        if self.first_time.is_none() {
            self.first_time = Some(entry.timestamp.clone());
        }
        self.last_time = Some(entry.timestamp.clone());
        if let Some(ms) = entry.duration_ms {
            self.total_ms += ms as u64;
            self.timed += 1;
            self.slowest.push(entry.clone());
            self.slowest.sort_by(|a, b| b.duration_ms.cmp(&a.duration_ms));
            self.slowest.truncate(KEEP_SLOWEST);
        }
        if entry.level == Level::Error {
            self.errors.push(entry);
        }
    }

    pub fn average_ms(&self) -> Option<f64> {
        (self.timed > 0).then(|| self.total_ms as f64 / self.timed as f64)
    }

    pub fn render(&self) -> String {
        let mut out = String::new();
        // write! into a String cannot fail, so unwrap is safe here.
        writeln!(out, "LOG REPORT").unwrap();
        writeln!(out, "==========").unwrap();
        writeln!(
            out,
            "Lines: {} total, {} parsed, {} skipped",
            self.total_lines,
            self.parsed,
            self.bad_lines.len()
        )
        .unwrap();
        if let (Some(first), Some(last)) = (&self.first_time, &self.last_time) {
            writeln!(out, "Time range: {first} to {last}").unwrap();
        }
        if let Some(avg) = self.average_ms() {
            writeln!(out, "Average duration: {avg:.1} ms").unwrap();
        }

        writeln!(out, "\nBy level:").unwrap();
        for (level, n) in &self.by_level {
            let bar = "#".repeat((n * 40).div_ceil(self.parsed.max(1)));
            writeln!(out, "  {level:<5} {n:>5}  {bar}").unwrap();
        }

        writeln!(out, "\nBusiest modules:").unwrap();
        let mut modules: Vec<_> = self.by_module.iter().collect();
        modules.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        for (module, n) in modules {
            writeln!(out, "  {module:<10} {n:>5}").unwrap();
        }

        writeln!(out, "\nSlowest requests:").unwrap();
        for e in &self.slowest {
            writeln!(out, "  {:>6} ms  [{}] {}", e.duration_ms.unwrap_or(0), e.module, e.message).unwrap();
        }

        writeln!(out, "\nErrors ({}):", self.errors.len()).unwrap();
        for e in &self.errors {
            writeln!(out, "  {}  [{}] {}", e.timestamp, e.module, e.message).unwrap();
        }

        if !self.bad_lines.is_empty() {
            writeln!(out, "\nSkipped lines:").unwrap();
            for (n, reason) in &self.bad_lines {
                writeln!(out, "  line {n}: {reason}").unwrap();
            }
        }
        out
    }
}
