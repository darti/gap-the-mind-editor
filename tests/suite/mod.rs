pub use super::test_markdown_html;

mod caret;

#[test]
fn table_test_1() {
    let original = r##"## Test header"##;
    let expected = r##"<h2>Test header</h2>
"##;

    test_markdown_html(original, expected, false);
}
