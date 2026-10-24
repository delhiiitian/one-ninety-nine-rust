// The command-line tool. All the real work is in the library (lib.rs);
// this file only handles arguments and files.
//
//   cargo run -p markdown-converter -- README.md              HTML to stdout
//   cargo run -p markdown-converter -- notes.md -o notes.html --page
//   echo "# Hi" | cargo run -p markdown-converter -- -

use std::error::Error;
use std::fs;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut input = None;
    let mut output = None;
    let mut page = false;
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "-o" | "--output" => output = Some(iter.next().ok_or("-o needs a file name")?.clone()),
            "--page" => page = true,
            "-h" | "--help" => {
                println!("usage: mdconv <file.md | -> [-o out.html] [--page]");
                return Ok(());
            }
            other => input = Some(other.to_string()),
        }
    }
    let input = input.ok_or("usage: mdconv <file.md | -> [-o out.html] [--page]")?;

    let markdown = if input == "-" {
        let mut text = String::new();
        io::stdin().read_to_string(&mut text)?;
        text
    } else {
        fs::read_to_string(&input).map_err(|e| format!("cannot read '{input}': {e}"))?
    };

    let body = mdconv::to_html(&markdown);
    let html = if page {
        // Use the first heading as the page title, if there is one.
        let title = mdconv::parse(&markdown)
            .into_iter()
            .find_map(|b| match b {
                mdconv::Block::Heading { text, .. } => Some(text),
                _ => None,
            })
            .unwrap_or_else(|| "Document".to_string());
        mdconv::full_page(&title, &body)
    } else {
        body
    };

    match output {
        Some(path) => {
            fs::write(&path, html)?;
            eprintln!("Wrote {path}");
        }
        None => print!("{html}"),
    }
    Ok(())
}
