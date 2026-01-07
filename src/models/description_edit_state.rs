#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DescriptionEditState {
    text_buffer: String,             // Single buffer with '\n' chars
    cursor_byte_offset: usize,       // Cursor position in bytes
    preferred_column: Option<usize>, // Remembers column for up/down nav
}

impl DescriptionEditState {
    pub fn new(initial_text: String) -> Self {
        let cursor_byte_offset = initial_text.len();
        Self {
            text_buffer: initial_text,
            cursor_byte_offset,
            preferred_column: None,
        }
    }

    pub fn text(&self) -> &str {
        &self.text_buffer
    }

    pub fn cursor_position(&self) -> (usize, usize) {
        self.offset_to_line_col(self.cursor_byte_offset)
    }

    fn offset_to_line_col(&self, offset: usize) -> (usize, usize) {
        let text_before = &self.text_buffer[..offset.min(self.text_buffer.len())];
        let line = text_before.chars().filter(|&c| c == '\n').count();
        let col = text_before
            .rsplit('\n')
            .next()
            .unwrap_or("")
            .chars()
            .count();
        (line, col)
    }

    fn line_col_to_offset(&self, target_line: usize, target_col: usize) -> usize {
        let mut current_line = 0;
        let mut current_offset = 0;

        for ch in self.text_buffer.chars() {
            if current_line == target_line {
                let mut col = 0;
                for c in self.text_buffer[current_offset..].chars() {
                    if c == '\n' {
                        break;
                    }
                    if col == target_col {
                        return current_offset;
                    }
                    current_offset += c.len_utf8();
                    col += 1;
                }
                // Reached end of line or end of text
                return current_offset;
            }

            if ch == '\n' {
                current_line += 1;
            }
            current_offset += ch.len_utf8();
        }

        // Target line doesn't exist, return end of buffer
        self.text_buffer.len()
    }

    fn get_line_at_offset(&self, offset: usize) -> usize {
        self.text_buffer[..offset.min(self.text_buffer.len())]
            .chars()
            .filter(|&c| c == '\n')
            .count()
    }

    fn get_column_at_offset(&self, offset: usize) -> usize {
        self.text_buffer[..offset.min(self.text_buffer.len())]
            .rsplit('\n')
            .next()
            .unwrap_or("")
            .chars()
            .count()
    }

    fn count_lines(&self) -> usize {
        if self.text_buffer.is_empty() {
            1
        } else {
            self.text_buffer.chars().filter(|&c| c == '\n').count() + 1
        }
    }

    // Public editing methods
    pub fn insert_char(&mut self, c: char) {
        self.text_buffer.insert(self.cursor_byte_offset, c);
        self.cursor_byte_offset += c.len_utf8();
        self.preferred_column = None;
    }

    pub fn insert_newline(&mut self) {
        self.insert_char('\n');
    }

    pub fn backspace(&mut self) {
        if self.cursor_byte_offset > 0 {
            let text = &self.text_buffer[..self.cursor_byte_offset];
            if let Some(ch) = text.chars().last() {
                let char_len = ch.len_utf8();
                self.text_buffer
                    .drain(self.cursor_byte_offset - char_len..self.cursor_byte_offset);
                self.cursor_byte_offset -= char_len;
                self.preferred_column = None;
            }
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_byte_offset > 0 {
            let text = &self.text_buffer[..self.cursor_byte_offset];
            if let Some(ch) = text.chars().last() {
                self.cursor_byte_offset -= ch.len_utf8();
                self.preferred_column = None;
            }
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_byte_offset < self.text_buffer.len() {
            let text = &self.text_buffer[self.cursor_byte_offset..];
            if let Some(ch) = text.chars().next() {
                self.cursor_byte_offset += ch.len_utf8();
                self.preferred_column = None;
            }
        }
    }

    pub fn move_cursor_up(&mut self) {
        let current_line = self.get_line_at_offset(self.cursor_byte_offset);
        if current_line == 0 {
            return;
        }

        let current_col = self.get_column_at_offset(self.cursor_byte_offset);
        let preferred = self.preferred_column.unwrap_or(current_col);
        self.preferred_column = Some(preferred);

        self.cursor_byte_offset = self.line_col_to_offset(current_line - 1, preferred);
    }

    pub fn move_cursor_down(&mut self) {
        let total_lines = self.count_lines();
        let current_line = self.get_line_at_offset(self.cursor_byte_offset);
        if current_line >= total_lines - 1 {
            return;
        }

        let current_col = self.get_column_at_offset(self.cursor_byte_offset);
        let preferred = self.preferred_column.unwrap_or(current_col);
        self.preferred_column = Some(preferred);

        self.cursor_byte_offset = self.line_col_to_offset(current_line + 1, preferred);
    }

    pub fn into_string(self) -> String {
        self.text_buffer
    }
}
