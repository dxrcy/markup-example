use regex::Regex;

use ListKind::*;
use Style::*;

/// Kind of line formatting style
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Style {
    /// Header with defined depth
    Header(usize),
    /// Block quote
    Quote,
    /// List with kind of `ListKind` - `Unordered` or `Ordered`
    List(ListKind),
    HorizontalLine,
    Comment,
}

impl Style {
    /// Split file line into optional line style and rest of line
    ///
    /// Returns `None` as style if token does not match
    pub fn from(line: &str) -> (Option<Self>, &str) {
        // Split line into token and rest of line
        let (token, rest_of_line) = match line.find(' ') {
            // Line contains space
            Some(position) => line.split_at(position),

            // Line contains no space
            // Use whole line as token, and rest of line blank
            None => (line, ""),
        };

        // Match token string to style enum
        let style = match token {
            // Headers
            s if Regex::new(r"^#+$").unwrap().is_match(s) => Header(s.len()),

            // Unordered list
            "-" => List(ListKind::Unordered),

            // Ordered list
            s if Regex::new(r"^\d+\.$").unwrap().is_match(s) => List(ListKind::Ordered),

            ">" | "&gt;" => Quote, // This must include html-escaped less-than character
            "---" => HorizontalLine,
            "~~~" => Comment,

            // Unknown token
            // Return no style and whole line
            _ => return (None, line.trim()),
        };

        // Return style as some, and rest of line
        (Some(style), rest_of_line.trim())
    }

    /// Format line using style kind
    pub fn format(self, line: &str) -> String {
        match self {
            // Use depth in html tag
            Header(n) => format!("<h{n}> {} </h{n}>", line),

            // Not affected by list kind
            List(_) => format!("<li> {} </li>", line),

            Quote => format!("<blockquote> {} </blockquote>", line),
            HorizontalLine => String::from("<hr />"),
            Comment => String::new(),
        }
    }
}

/// Kind of list for `Style` enum
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ListKind {
    /// Unordered list variant
    Unordered,
    /// Ordered list variant
    Ordered,
}

impl ListKind {
    /// Get opening html tag from kind
    pub fn opening_tag(&self) -> &'static str {
        match self {
            Unordered => "<ul>",
            Ordered => "<ol>",
        }
    }

    /// Get closing html tag from kind
    pub fn closing_tag(&self) -> &'static str {
        match self {
            Unordered => "</ul>",
            Ordered => "</ol>",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn style_from_works() {
        // Headers
        assert_eq!(Style::from("# Hello"), (Some(Header(1)), "Hello"));
        assert_eq!(Style::from("## Hello"), (Some(Header(2)), "Hello"));
        assert_eq!(Style::from("### Hello"), (Some(Header(3)), "Hello"));

        // Invalid header (no space)
        assert_eq!(Style::from("#Hello"), (None, "#Hello"));
        // Header with another hashtag
        assert_eq!(Style::from("# # Hello"), (Some(Header(1)), "# Hello"));

        // Block quote
        assert_eq!(Style::from("> Hello"), (Some(Quote), "Hello"));

        // Horizontal line
        assert_eq!(Style::from("---"), (Some(HorizontalLine), ""));
        assert_eq!(Style::from("--- Hello"), (Some(HorizontalLine), "Hello"));

        // Comment
        assert_eq!(Style::from("~~~ Hello"), (Some(Comment), "Hello"));
        assert_eq!(Style::from("~~~Hello"), (None, "~~~Hello"));

        // No token (including unknown)
        assert_eq!(Style::from("Hello"), (None, "Hello"));
        assert_eq!(Style::from("& Hello"), (None, "& Hello"));
    }

    #[test]
    fn style_format_works() {
        // Headers
        assert_eq!(Header(1).format("Hello"), "<h1> Hello </h1>");
        assert_eq!(Header(2).format("Hello"), "<h2> Hello </h2>");
        assert_eq!(Header(3).format("Hello"), "<h3> Hello </h3>");
        assert_eq!(Header(4).format("Hello"), "<h4> Hello </h4>");

        // Lists (not affected by list kind)
        assert_eq!(List(Ordered).format("Hello"), "<li> Hello </li>");
        assert_eq!(List(Unordered).format("Hello"), "<li> Hello </li>");

        // Other
        assert_eq!(Quote.format("Hello"), "<blockquote> Hello </blockquote>");
        assert_eq!(HorizontalLine.format("Hello"), "<hr />");
        assert_eq!(Comment.format("Hello"), "");
    }
}
