# Project 6: Log file analyzer

Reads a server log of any size line by line and reports counts per level,
the busiest modules, the slowest requests, every error, and lines it could
not understand. Filters by level and module, reads files or standard input,
and can save the report to a file. Built in Chapter 6 of *Rust in 199 Pages*.

```bash
cargo run -p log-analyzer -- projects/06-log-analyzer/sample.log
cargo run -p log-analyzer -- projects/06-log-analyzer/sample.log --level warn
cargo run -p log-analyzer -- projects/06-log-analyzer/sample.log --module payment --output report.txt
cat projects/06-log-analyzer/sample.log | cargo run -p log-analyzer -- -
cargo test -p log-analyzer
```

Log line format:

```text
2026-10-24T10:15:32 INFO  [auth] User asha logged in (12ms)
```

## Layout

```text
06-log-analyzer/
├── sample.log      122 lines to try it on (including two bad ones)
├── src/
│   ├── main.rs     options, opening the input, writing the output
│   ├── parser.rs   Level, LogEntry, parse_line, tests
│   └── report.rs   Stats: collect while streaming, render the report
└── tests/
    └── cli.rs      runs the real program on sample.log
```

## Ideas to extend

- `--since 10:30` and `--until 11:00` time filters.
- Count errors per minute and print the worst minute.
- Watch a file and update the report as new lines arrive (like `tail -f`).
