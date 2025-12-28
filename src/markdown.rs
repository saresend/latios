use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

/// Parse markdown text and convert it to styled ratatui Lines
pub fn parse_markdown(text: &str) -> Vec<Line<'static>> {
    let parser = Parser::new(text);
    let mut lines = Vec::new();
    let mut current_line = Vec::new();
    let mut current_style = Style::default();
    let mut list_depth: u32 = 0;
    let mut in_code_block = false;
    let mut heading_level: Option<HeadingLevel> = None;

    for event in parser {
        match event {
            Event::Start(tag) => {
                current_style = match tag {
                    Tag::Strong => current_style.add_modifier(Modifier::BOLD),
                    Tag::Emphasis => current_style.add_modifier(Modifier::ITALIC),
                    Tag::Heading { level, .. } => {
                        heading_level = Some(level);
                        match level {
                            HeadingLevel::H1 | HeadingLevel::H2 => {
                                Style::default()
                                    .add_modifier(Modifier::BOLD)
                                    .fg(Color::Cyan)
                            }
                            _ => Style::default().add_modifier(Modifier::BOLD).fg(Color::Blue),
                        }
                    }
                    Tag::CodeBlock(_) => {
                        in_code_block = true;
                        Style::default().fg(Color::Yellow).bg(Color::DarkGray)
                    }
                    Tag::Link { .. } => current_style
                        .add_modifier(Modifier::UNDERLINED)
                        .fg(Color::Blue),
                    Tag::BlockQuote(_) => current_style.fg(Color::DarkGray),
                    Tag::List(_) => {
                        list_depth += 1;
                        current_style
                    }
                    _ => current_style,
                };
            }
            Event::End(tag_end) => {
                match tag_end {
                    TagEnd::Heading(_) => {
                        // End of heading - flush current line and add blank line
                        if !current_line.is_empty() {
                            lines.push(Line::from(current_line.clone()));
                            current_line.clear();
                        }
                        lines.push(Line::from("")); // Blank line after heading
                        heading_level = None;
                        current_style = Style::default();
                    }
                    TagEnd::Paragraph => {
                        // End of paragraph - flush current line
                        if !current_line.is_empty() {
                            lines.push(Line::from(current_line.clone()));
                            current_line.clear();
                        }
                        if !in_code_block {
                            lines.push(Line::from("")); // Blank line after paragraph
                        }
                        current_style = Style::default();
                    }
                    TagEnd::CodeBlock => {
                        if !current_line.is_empty() {
                            lines.push(Line::from(current_line.clone()));
                            current_line.clear();
                        }
                        lines.push(Line::from("")); // Blank line after code block
                        in_code_block = false;
                        current_style = Style::default();
                    }
                    TagEnd::List(_) => {
                        list_depth = list_depth.saturating_sub(1);
                        if list_depth == 0 && !current_line.is_empty() {
                            lines.push(Line::from(current_line.clone()));
                            current_line.clear();
                            lines.push(Line::from("")); // Blank line after list
                        }
                        current_style = Style::default();
                    }
                    TagEnd::Item => {
                        // End of list item - flush current line
                        if !current_line.is_empty() {
                            lines.push(Line::from(current_line.clone()));
                            current_line.clear();
                        }
                        current_style = Style::default();
                    }
                    TagEnd::Strong | TagEnd::Emphasis => {
                        current_style = Style::default();
                    }
                    TagEnd::Link => {
                        current_style = Style::default();
                    }
                    _ => {}
                }
            }
            Event::Text(text) => {
                let text_str = text.to_string();

                // Handle code differently based on context
                if in_code_block {
                    // Code block: preserve newlines
                    for line in text_str.lines() {
                        if !current_line.is_empty() || !line.is_empty() {
                            current_line.push(Span::styled(
                                line.to_string(),
                                Style::default().fg(Color::Yellow).bg(Color::DarkGray),
                            ));
                            lines.push(Line::from(current_line.clone()));
                            current_line.clear();
                        }
                    }
                } else {
                    // Regular text: apply current style
                    current_line.push(Span::styled(text_str, current_style));
                }
            }
            Event::Code(code) => {
                // Inline code
                current_line.push(Span::styled(
                    code.to_string(),
                    Style::default().fg(Color::Yellow).bg(Color::DarkGray),
                ));
            }
            Event::SoftBreak | Event::HardBreak => {
                if !current_line.is_empty() {
                    lines.push(Line::from(current_line.clone()));
                    current_line.clear();
                }
            }
            Event::Rule => {
                // Horizontal rule
                lines.push(Line::from(Span::styled(
                    "─".repeat(50),
                    Style::default().fg(Color::DarkGray),
                )));
                lines.push(Line::from(""));
            }
            Event::TaskListMarker(checked) => {
                let marker = if checked { "[✓] " } else { "[ ] " };
                current_line.push(Span::raw(marker));
            }
            _ => {}
        }
    }

    // Flush any remaining content
    if !current_line.is_empty() {
        lines.push(Line::from(current_line));
    }

    // If no lines were generated, return empty vector
    // The caller will handle showing "(No description)"
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_markdown() {
        let lines = parse_markdown("");
        assert_eq!(lines.len(), 0);
    }

    #[test]
    fn test_simple_text() {
        let lines = parse_markdown("Hello world");
        assert!(lines.len() > 0);
    }

    #[test]
    fn test_bold() {
        let lines = parse_markdown("This is **bold** text");
        assert!(lines.len() > 0);
    }

    #[test]
    fn test_heading() {
        let lines = parse_markdown("# Heading\nSome text");
        assert!(lines.len() >= 2);
    }

    #[test]
    fn test_code() {
        let lines = parse_markdown("Use `code` here");
        assert!(lines.len() > 0);
    }
}
