// Block structure: which lines form headings, lists, quotes, code...
// Parsing produces a Vec<Block>; rendering turns each Block into HTML.
// Keeping the two apart means other tools can use the structure too.

use crate::inline;

/// One block-level piece of a document.
#[derive(Debug, Clone, PartialEq)]
pub enum Block {
    Heading { level: u8, text: String },
    Paragraph(String),
    BulletList(Vec<String>),
    NumberedList(Vec<String>),
    Quote(String),
    Code { language: Option<String>, code: String },
    Rule,
}

pub fn parse(markdown: &str) -> Vec<Block> {
    let lines: Vec<&str> = markdown.lines().collect();
    let mut blocks = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        if trimmed.is_empty() {
            i += 1;
        } else if let Some(rest) = trimmed.strip_prefix("```") {
            // Fenced code block: collect until the closing ```
            let language = (!rest.is_empty()).then(|| rest.to_string());
            let mut code = Vec::new();
            i += 1;
            while i < lines.len() && !lines[i].trim_start().starts_with("```") {
                code.push(lines[i]);
                i += 1;
            }
            i += 1; // skip the closing fence
            blocks.push(Block::Code { language, code: code.join("\n") });
        } else if let Some((level, text)) = heading(trimmed) {
            blocks.push(Block::Heading { level, text: text.to_string() });
            i += 1;
        } else if trimmed == "---" || trimmed == "***" {
            blocks.push(Block::Rule);
            i += 1;
        } else if bullet(trimmed).is_some() {
            let mut items = Vec::new();
            while i < lines.len() {
                let Some(item) = bullet(lines[i].trim()) else { break };
                items.push(item.to_string());
                i += 1;
            }
            blocks.push(Block::BulletList(items));
        } else if numbered(trimmed).is_some() {
            let mut items = Vec::new();
            while i < lines.len() {
                let Some(item) = numbered(lines[i].trim()) else { break };
                items.push(item.to_string());
                i += 1;
            }
            blocks.push(Block::NumberedList(items));
        } else if trimmed.starts_with('>') {
            let mut parts = Vec::new();
            while i < lines.len() && lines[i].trim().starts_with('>') {
                parts.push(lines[i].trim().trim_start_matches('>').trim());
                i += 1;
            }
            blocks.push(Block::Quote(parts.join(" ")));
        } else {
            // A paragraph runs until a blank line or the start of another block.
            let mut parts = Vec::new();
            while i < lines.len() && !lines[i].trim().is_empty() && !starts_block(lines[i].trim()) {
                parts.push(lines[i].trim());
                i += 1;
            }
            blocks.push(Block::Paragraph(parts.join(" ")));
        }
    }
    blocks
}

fn heading(line: &str) -> Option<(u8, &str)> {
    let hashes = line.chars().take_while(|&c| c == '#').count();
    if (1..=6).contains(&hashes) && line[hashes..].starts_with(' ') {
        Some((hashes as u8, line[hashes..].trim()))
    } else {
        None
    }
}

fn bullet(line: &str) -> Option<&str> {
    line.strip_prefix("- ").or_else(|| line.strip_prefix("* "))
}

fn numbered(line: &str) -> Option<&str> {
    let (number, rest) = line.split_once(". ")?;
    (!number.is_empty() && number.chars().all(|c| c.is_ascii_digit())).then_some(rest)
}

fn starts_block(line: &str) -> bool {
    heading(line).is_some()
        || bullet(line).is_some()
        || numbered(line).is_some()
        || line.starts_with('>')
        || line.starts_with("```")
        || line == "---"
}

pub fn render(block: &Block) -> String {
    match block {
        Block::Heading { level, text } => format!("<h{level}>{}</h{level}>\n", inline::render(text)),
        Block::Paragraph(text) => format!("<p>{}</p>\n", inline::render(text)),
        Block::BulletList(items) => list("ul", items),
        Block::NumberedList(items) => list("ol", items),
        Block::Quote(text) => format!("<blockquote><p>{}</p></blockquote>\n", inline::render(text)),
        Block::Code { language, code } => {
            let class = match language {
                Some(lang) => format!(" class=\"language-{}\"", inline::escape(lang)),
                None => String::new(),
            };
            format!("<pre><code{class}>{}</code></pre>\n", inline::escape(code))
        }
        Block::Rule => "<hr>\n".to_string(),
    }
}

fn list(tag: &str, items: &[String]) -> String {
    let mut html = format!("<{tag}>\n");
    for item in items {
        html.push_str(&format!("<li>{}</li>\n", inline::render(item)));
    }
    html.push_str(&format!("</{tag}>\n"));
    html
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognises_blocks() {
        let doc = "# Title\n\nPara one\ncontinues.\n\n- a\n- b\n\n1. x\n2. y\n\n> quote\n\n---\n\n```rust\nlet x = 1 < 2;\n```";
        let blocks = parse(doc);
        assert_eq!(blocks[0], Block::Heading { level: 1, text: "Title".into() });
        assert_eq!(blocks[1], Block::Paragraph("Para one continues.".into()));
        assert_eq!(blocks[2], Block::BulletList(vec!["a".into(), "b".into()]));
        assert_eq!(blocks[3], Block::NumberedList(vec!["x".into(), "y".into()]));
        assert_eq!(blocks[4], Block::Quote("quote".into()));
        assert_eq!(blocks[5], Block::Rule);
        assert_eq!(blocks[6], Block::Code { language: Some("rust".into()), code: "let x = 1 < 2;".into() });
    }

    #[test]
    fn not_headings() {
        assert_eq!(heading("#hashtag"), None);
        assert_eq!(heading("####### seven"), None);
        assert_eq!(numbered("2026. was a year"), Some("was a year"));
        assert_eq!(numbered("v1. nope"), None);
    }
}
