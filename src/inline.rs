/// Whether each inline format is active
#[derive(Default)]
struct Inlines {
    pub italic: bool,
    pub bold: bool,
    pub underline: bool,
    pub code: bool,
}

/// Format inline styles of line
pub fn format_inline_styles(line: &str) -> String {
    // Current active inlines
    let mut current_inlines = Inlines::default();
    // If character is escaped
    let mut is_char_escaped = false;

    // Returned string
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
                    formatted_line.push_str(if current_inlines.italic { "</i>" } else { "<i>" });
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

                // Any other character
                _ => formatted_line.push(ch),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_works() {
        // Normal inlines
        assert_eq!(format_inline_styles("Some *example* text"), "Some <i>example</i> text");
        assert_eq!(format_inline_styles("Some ^example^ text"), "Some <b>example</b> text");
        assert_eq!(format_inline_styles("Some _example_ text"), "Some <u>example</u> text");
        assert_eq!(
            format_inline_styles("Some `example` text"),
            "Some <code>example</code> text"
        );

        // Nested inlines
        assert_eq!(
            format_inline_styles("Some *example ^text^*"),
            "Some <i>example <b>text</b></i>"
        );
        assert_eq!(
            format_inline_styles("Some *^example^* text"),
            "Some <i><b>example</b></i> text"
        );
        assert_eq!(
            format_inline_styles("_Some *example_ ^text^*"),
            "<u>Some <i>example</u> <b>text</b></i>"
        );

        // Escaped backslash
        assert_eq!(format_inline_styles(r"Some \\ example text"), r"Some \ example text");

        // Escaped inlines
        assert_eq!(format_inline_styles(r"Some \*example\* text"), "Some *example* text");
        assert_eq!(
            format_inline_styles(r"Some \^\_example\_\^ text"),
            "Some ^_example_^ text"
        );
    }
}
