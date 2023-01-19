use crate::find_back;

/// Whether each inline format is active
#[derive(Default)]
struct Inlines {
    pub italic: bool,
    pub bold: bool,
    pub underline: bool,
    pub code: bool,
}

/// Format inline styles and links of line
pub fn format_inlines(line: &str) -> String {
    // If character is escaped
    let mut is_char_escaped = false;
    // Current active inlines
    let mut current_inlines = Inlines::default();
    // Current building link
    let mut current_link: Option<String> = None;

    // Return string
    let mut formatted_line = String::new();

    // Loop characters
    for ch in line.chars() {
        if is_char_escaped {
            // Ignore special uses if escaped
            formatted_line.push(ch);
        } else {
            // Open or close inline tags, change current inlines, if character is special
            match ch {
                // Backslash, not escaped
                '\\' => (),

                // Italic
                '*' => {
                    formatted_line.push_str(if current_inlines.italic {
                        "</i>"
                    } else {
                        "<i>"
                    });
                    current_inlines.italic ^= true;
                }

                // Bold
                '^' => {
                    formatted_line.push_str(if current_inlines.bold { "</b>" } else { "<b>" });
                    current_inlines.bold ^= true;
                }

                // Underline
                '_' => {
                    formatted_line.push_str(if current_inlines.underline {
                        "</u>"
                    } else {
                        "<u>"
                    });
                    current_inlines.underline ^= true;
                }

                // Code
                '`' => {
                    formatted_line.push_str(if current_inlines.code {
                        "</code>"
                    } else {
                        "<code>"
                    });
                    current_inlines.code ^= true;
                }

                // Start link
                '[' if current_link.is_none() => current_link = Some(String::new()),

                // End link
                ']' if current_link.is_some() => {
                    // Add link to formatted line
                    if let Some(link) = current_link {
                        let (content, href) = separate_link_content(&link);
                        formatted_line.push_str(&format!(r#"<a href="{href}">{}</a>"#, content));
                    }
                    // Reset link
                    current_link = None;
                }

                // Any other character
                _ => {
                    if let Some(link) = &mut current_link {
                        link.push(ch);
                    } else {
                        formatted_line.push(ch)
                    }
                }
            }
        }

        // Character is backslash, and not escaped
        // Escape next character
        if ch == '\\' && !is_char_escaped {
            is_char_escaped = true;
        } else {
            is_char_escaped = false;
        }
    }

    formatted_line
}

/// Split raw link content into href and text content
///
/// Separates at pipe `|`
fn separate_link_content(link: &str) -> (&str, &str) {
    if let Some(pos) = find_back(link, '|') {
        let (a, b) = link.split_at(pos);
        (a.trim(), remove_first_char(b).trim())
    } else {
        (link, "")
    }
}

/// Remove first character of string slice
fn remove_first_char(s: &str) -> &str {
    let mut chars = s.chars();
    chars.next();
    chars.as_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_works() {
        // Normal inlines
        assert_eq!(
            format_inlines("Some *example* text"),
            "Some <i>example</i> text"
        );
        assert_eq!(
            format_inlines("Some ^example^ text"),
            "Some <b>example</b> text"
        );
        assert_eq!(
            format_inlines("Some _example_ text"),
            "Some <u>example</u> text"
        );
        assert_eq!(
            format_inlines("Some `example` text"),
            "Some <code>example</code> text"
        );

        // Nested inlines
        assert_eq!(
            format_inlines("Some *example ^text^*"),
            "Some <i>example <b>text</b></i>"
        );
        assert_eq!(
            format_inlines("Some *^example^* text"),
            "Some <i><b>example</b></i> text"
        );
        assert_eq!(
            format_inlines("_Some *example_ ^text^*"),
            "<u>Some <i>example</u> <b>text</b></i>"
        );

        // Escaped backslash
        assert_eq!(
            format_inlines(r"Some \\ example text"),
            r"Some \ example text"
        );

        // Escaped inlines
        assert_eq!(
            format_inlines(r"Some \*example\* text"),
            "Some *example* text"
        );
        assert_eq!(
            format_inlines(r"Some \^\_example\_\^ text"),
            "Some ^_example_^ text"
        );
    }

    #[test]
    fn separate_link_content_works() {
        assert_eq!(
            separate_link_content("link content | https://example.com"),
            ("link content", "https://example.com")
        );

        assert_eq!(
            separate_link_content("link | example | content | https://example.com"),
            ("link | example | content", "https://example.com")
        );

        assert_eq!(separate_link_content("link content"), ("link content", ""));
    }

    #[test]
    fn format_links_works() {
        assert_eq!(
            format_inlines("[link content | https://example.com]"),
            r#"<a href="https://example.com"> link content </a>"#
        );

        assert_eq!(
            format_inlines("[link | example | content | https://example.com]"),
            r#"<a href="https://example.com"> link | example | content </a>"#
        );

        assert_eq!(
            format_inlines("[link content]"),
            r#"<a href=""> link content </a>"#
        );

        assert_eq!(format_inlines(r"\[no link\]"), r#"[no link]"#);
    }
}
