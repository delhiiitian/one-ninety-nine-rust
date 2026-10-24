// Project 6: Log file analyzer
//
//   cargo run -p log-analyzer -- projects/06-log-analyzer/sample.log
//   cargo run -p log-analyzer -- app.log --level warn --module payment
//   cargo run -p log-analyzer -- app.log --output report.txt
//   cat app.log | cargo run -p log-analyzer -- -        (read standard input)

mod parser;
mod report;

use parser::Level;
use report::Stats;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

struct Options {
    input: String,
    min_level: Level,
    module: Option<String>,
    output: Option<String>,
}

fn parse_args(args: &[String]) -> Result<Options, String> {
    let mut input = None;
    let mut min_level = Level::Debug;
    let mut module = None;
    let mut output = None;
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--level" => {
                let text = iter.next().ok_or("--level needs a value")?;
                min_level = Level::parse(text).ok_or(format!("unknown level '{text}'"))?;
            }
            "--module" => module = Some(iter.next().ok_or("--module needs a value")?.clone()),
            "--output" => output = Some(iter.next().ok_or("--output needs a file name")?.clone()),
            flag if flag.starts_with("--") => return Err(format!("unknown option '{flag}'")),
            path => input = Some(path.to_string()),
        }
    }
    let input = input.ok_or("usage: log-analyzer <file|-> [--level LEVEL] [--module NAME] [--output FILE]")?;
    Ok(Options { input, min_level, module, output })
}

/// Reads entries from any buffered source: a file or standard input.
fn analyze(reader: impl BufRead, options: &Options) -> io::Result<Stats> {
    let mut stats = Stats::default();
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        stats.total_lines += 1;
        if line.trim().is_empty() {
            continue;
        }
        match parser::parse_line(&line) {
            Ok(entry) => {
                let level_ok = entry.level >= options.min_level;
                let module_ok = options.module.as_ref().is_none_or(|m| *m == entry.module);
                if level_ok && module_ok {
                    stats.add(entry);
                }
            }
            Err(reason) => stats.bad_lines.push((index + 1, reason)),
        }
    }
    Ok(stats)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let options = parse_args(&args)?;

    let stats = if options.input == "-" {
        analyze(io::stdin().lock(), &options)?
    } else {
        let file = File::open(&options.input).map_err(|e| format!("cannot open '{}': {e}", options.input))?;
        analyze(BufReader::new(file), &options)?
    };

    let report = stats.render();
    match &options.output {
        Some(path) => {
            let mut file = File::create(path)?;
            file.write_all(report.as_bytes())?;
            println!("Report written to {path} ({} entries analysed)", stats.parsed);
        }
        None => print!("{report}"),
    }
    Ok(())
}
