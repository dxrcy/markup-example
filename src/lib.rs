/// Inline styles
mod inline;
/// Whole line styles
mod style;

use crate::{
    inline::format_inline_styles,
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

        // Format line with inline styles
        let line = format_inline_styles(line);

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
            Style::no_format(&line)
        };

        // Add line to body
        body.push(formatted_line);
    }

    // Close final list, if active
    if let Some(current) = current_list {
        body.push(current.closing_tag().into());
    }

    // Complete template with body
    let html = include_str!("template.html")
        .replace("{{BODY}}", &body.join("\n"))
        .replace("{{TITLE}}", &title.unwrap_or("Markup File".to_string()));

    // Return html, including template
    Ok(html)
}
