// Inline formatting inside one block of text:
//   **bold**  *italic*  `code`  [text](url)
// plus escaping of < > & " so text can never inject HTML.

/// Makes text safe to put inside HTML.
pub fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    out
}

/// Renders inline Markdown to HTML.
pub fn render(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < chars.len() {
        // `code`: everything inside is literal
        if chars[i] == '`' {
            if let Some(end) = find(&chars, i + 1, "`") {
                let inner: String = chars[i + 1..end].iter().collect();
                out.push_str(&format!("<code>{}</code>", escape(&inner)));
                i = end + 1;
                continue;
            }
        }
        // **bold**
        if starts_with(&chars, i, "**") {
            if let Some(end) = find(&chars, i + 2, "**") {
                let inner: String = chars[i + 2..end].iter().collect();
                out.push_str(&format!("<strong>{}</strong>", render(&inner)));
                i = end + 2;
                continue;
            }
        }
        // *italic*
        if chars[i] == '*' {
            if let Some(end) = find(&chars, i + 1, "*") {
                let inner: String = chars[i + 1..end].iter().collect();
                out.push_str(&format!("<em>{}</em>", render(&inner)));
                i = end + 1;
                continue;
            }
        }
        // [text](url)
        if chars[i] == '[' {
            if let Some(close) = find(&chars, i + 1, "](") {
                if let Some(end) = find(&chars, close + 2, ")") {
                    let label: String = chars[i + 1..close].iter().collect();
                    let url: String = chars[close + 2..end].iter().collect();
                    out.push_str(&format!("<a href=\"{}\">{}</a>", escape(&url), render(&label)));
                    i = end + 1;
                    continue;
                }
            }
        }
        out.push_str(&escape(&chars[i].to_string()));
        i += 1;
    }
    out
}

fn starts_with(chars: &[char], at: usize, pattern: &str) -> bool {
    let p: Vec<char> = pattern.chars().collect();
    chars.len() >= at + p.len() && chars[at..at + p.len()] == p[..]
}

/// Position of the next `pattern` at or after `from`, if any.
fn find(chars: &[char], from: usize, pattern: &str) -> Option<usize> {
    (from..chars.len()).find(|&i| starts_with(chars, i, pattern))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatting() {
        assert_eq!(render("a **b** c"), "a <strong>b</strong> c");
        assert_eq!(render("*it*"), "<em>it</em>");
        assert_eq!(render("**bold** and *italic*"), "<strong>bold</strong> and <em>italic</em>");
        assert_eq!(render("**a [link](u)**"), "<strong>a <a href=\"u\">link</a></strong>");
        assert_eq!(render("use `Vec<T>`"), "use <code>Vec&lt;T&gt;</code>");
    }

    #[test]
    fn links_and_escaping() {
        assert_eq!(render("[Rust](https://rust-lang.org)"), "<a href=\"https://rust-lang.org\">Rust</a>");
        assert_eq!(render("<script>"), "&lt;script&gt;");
        assert_eq!(render("2 * 3"), "2 * 3"); // a lone star stays a star
    }
}
