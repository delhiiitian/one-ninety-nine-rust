//! # mdconv
//!
//! Converts a useful subset of Markdown to HTML:
//! headings, paragraphs, bold, italic, inline code, links,
//! bullet and numbered lists, block quotes, code blocks and rules.
//!
//! ```
//! let html = mdconv::to_html("# Hello\n\nSome **bold** text.");
//! assert_eq!(html, "<h1>Hello</h1>\n<p>Some <strong>bold</strong> text.</p>\n");
//! ```

mod block;
mod inline;

pub use block::Block;

/// Converts Markdown text to an HTML fragment.
///
/// ```
/// let html = mdconv::to_html("- one\n- two");
/// assert_eq!(html, "<ul>\n<li>one</li>\n<li>two</li>\n</ul>\n");
/// ```
pub fn to_html(markdown: &str) -> String {
    let blocks = block::parse(markdown);
    blocks.iter().map(block::render).collect()
}

/// Wraps an HTML fragment in a complete page with a title.
///
/// ```
/// let page = mdconv::full_page("Notes", "<p>hi</p>\n");
/// assert!(page.starts_with("<!DOCTYPE html>"));
/// assert!(page.contains("<title>Notes</title>"));
/// ```
pub fn full_page(title: &str, body: &str) -> String {
    format!(
        "<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"utf-8\">\n<title>{}</title>\n\
         <style>body{{font-family:sans-serif;max-width:40em;margin:2em auto;line-height:1.5}}\
         code{{background:#eee;padding:0 .2em}}pre{{background:#eee;padding:1em}}</style>\n\
         </head>\n<body>\n{body}</body>\n</html>\n",
        inline::escape(title)
    )
}

/// Counts the headings, a small example of using the public `Block` type.
///
/// ```
/// let blocks = mdconv::parse("# A\n## B\ntext");
/// assert_eq!(mdconv::heading_count(&blocks), 2);
/// ```
pub fn heading_count(blocks: &[Block]) -> usize {
    blocks.iter().filter(|b| matches!(b, Block::Heading { .. })).count()
}

/// Parses Markdown into blocks without rendering, for tools that want the structure.
pub fn parse(markdown: &str) -> Vec<Block> {
    block::parse(markdown)
}
