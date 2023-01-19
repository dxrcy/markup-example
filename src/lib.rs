/// Inline styles
mod inline;
/// Whole line styles
mod style;

use crate::{
    inline::format_inlines,
    style::{
        ListKind,
        Style::{self, *},
    },
};

/// Compile a markup file to html
pub fn compile(file: &str) -> Result<String, String> {
    // Sanitize (escape) html
    let file = html_escape::encode_text(file);

    // Html body contents
    let mut body = Vec::<String>::new();

    // Title of page
    let mut title: Option<String> = None;

    // Currently active list kind
    let mut current_list: Option<ListKind> = None;

    for line in file.lines() {
        // Split line into token and rest of line
        let (style, line) = Style::from(line);

        // Format line with inline styles and links
        let line = format_inlines(line);

        // If style is None or style is not list
        // Unwrap should never fail
        if style.is_none() || !matches!(style.as_ref().unwrap(), List(_)) {
            // If current list is active
            if let Some(current) = current_list {
                body.push(current.closing_tag().into());
                current_list = None;
            }
        }

        // Format line with line token, if token is Some
        let formatted_line = if let Some(style) = style {
            // If current style is a list
            if let List(style) = &style {
                // If current list is active
                if let Some(current) = &current_list {
                    // If list types do not match
                    if current != style {
                        // Close previous list
                        body.push(current.closing_tag().into());
                    }
                }

                // If current list is not active, or current list does not match style list
                // Unwrap should never fail
                if current_list.is_none() || &current_list.unwrap() != style {
                    // Open new list
                    body.push(style.opening_tag().into());
                }
            }

            // Set current list to style (if is list), otherwise None
            current_list = match &style {
                List(list) => Some(*list),
                _ => None,
            };

            // Format line with style
            let formatted_line = style.format(&line);

            // Set title, if not set, and is <h1>
            if title.is_none() {
                if let Header(1) = style {
                    title = Some(line);
                }
            }

            formatted_line
        } else {
            // Skip blank lines
            if line.is_empty() {
                continue;
            }

            // Default formatting
            Some(Style::no_format(&line))
        };

        // Add line to body if not None
        if let Some(formatted_line) = formatted_line {
            body.push(formatted_line);
        }
    }

    // Close final list, if active
    if let Some(current) = current_list {
        body.push(current.closing_tag().into());
    }

    // Complete template with body
    let html = include_str!("template.html")
        .replace("{{BODY}}", &body.join("\n    "))
        .replace("{{TITLE}}", &title.unwrap_or("Markup File".to_string()));

    // Return html, including template
    Ok(html)
}

/// Find position of character in string, from back
///
/// Similar to String::find, but reverse
pub fn find_back(s: &str, c: char) -> Option<usize> {
    for (i, ch) in s.chars().rev().enumerate() {
        if ch == c {
            return s.len().checked_sub(i + 1);
        }
    }

    None
}

/// Change filename extension to another string
///
/// All characters after last dot are included in extension
///
/// All characters (including dots), except everything after last dot, are included in filename
pub fn replace_file_extension(filename: &str, extension: &str) -> String {
    // Find position of last period
    let filename = match find_back(filename, '.') {
        // Split whole filename at that position
        Some(pos) => filename.split_at(pos).0,
        // No period - Use whole filename
        None => filename,
    };

    // No new extension - return just filename, without original extension
    if extension.is_empty() {
        return filename.to_string();
    }

    // Return filename with new extension
    filename.to_string() + "." + extension
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_back_works() {
        assert_eq!(find_back("abcd", 'a'), Some(0));
        assert_eq!(find_back("abcd", 'b'), Some(1));
        assert_eq!(find_back("abcd", 'c'), Some(2));
        assert_eq!(find_back("abcd", 'd'), Some(3));
        assert_eq!(find_back("abcd", 'e'), None);
        assert_eq!(find_back("abad", 'a'), Some(2));
    }

    #[test]
    fn replace_file_extension_works() {
        use super::replace_file_extension as rfe;

        assert_eq!(rfe("abc", "html"), "abc.html");
        assert_eq!(rfe("abc.txt", "html"), "abc.html");
        assert_eq!(rfe("abc.def.txt", "html"), "abc.def.html");
        assert_eq!(rfe("abc.def.ghi.txt", "html"), "abc.def.ghi.html");

        assert_eq!(rfe("abc", ""), "abc");
        assert_eq!(rfe("abc.txt", ""), "abc");
    }
}
