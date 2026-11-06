// Integration tests use the library exactly as another crate would:
// only the public items (`to_html`, `full_page`, `parse`, `Block`) are visible.

#[test]
fn converts_a_small_document() {
    let md = "# Shopping\n\nBuy *fresh* mangoes from [the market](https://example.com).\n\n- mangoes\n- **jaggery**\n";
    let expected = "<h1>Shopping</h1>\n\
                    <p>Buy <em>fresh</em> mangoes from <a href=\"https://example.com\">the market</a>.</p>\n\
                    <ul>\n<li>mangoes</li>\n<li><strong>jaggery</strong></li>\n</ul>\n";
    assert_eq!(mdconv::to_html(md), expected);
}

#[test]
fn html_in_input_is_escaped() {
    let html = mdconv::to_html("Hello <b>world</b> & friends");
    assert_eq!(html, "<p>Hello &lt;b&gt;world&lt;/b&gt; &amp; friends</p>\n");
}

#[test]
fn sample_file_converts() {
    let md = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/sample.md")).unwrap();
    let html = mdconv::to_html(&md);
    assert!(html.contains("<h1>"));
    assert!(html.contains("<pre><code class=\"language-rust\">"));
    assert!(html.contains("<ol>"));
}
