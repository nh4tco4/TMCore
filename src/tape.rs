use std::collections::HashMap;
use std::ops::RangeInclusive;

pub const SPACE: char = ' ';
pub const LAMBDA: char = 'λ';

pub struct Tape {
    items: HashMap<i32, char>,
}

impl Tape {
    /// Creates an empty tape.
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    /// Creates a tape from an iterator of chars, starting at index 0.
    pub fn from_iter(chars: impl IntoIterator<Item = char>) -> Self {
        let items = chars
            .into_iter()
            .enumerate()
            .map(|(i, c)| (i as i32, c))
            .collect();
        Self { items }
    }

    /// Returns `Some(char)` if the cell exists, `None` otherwise.
    pub fn peek(&self, index: i32) -> Option<char> {
        self.items.get(&index).copied()
    }

    /// Inserts SPACE at `index` if not already present.
    pub fn extend_tape(&mut self, index: i32) {
        self.items.entry(index).or_insert(SPACE);
    }

    /// Writes `value` at `index`.
    pub fn write(&mut self, value: char, index: i32) {
        self.items.insert(index, value);
    }

    /// Reads cell; returns `SPACE` if cell is absent.
    pub fn read_or_space(&self, index: i32) -> char {
        self.items.get(&index).copied().unwrap_or(SPACE)
    }

    /// Reads cell; returns `LAMBDA` if cell is absent or contains SPACE.
    pub fn read_or_lambda(&self, index: i32) -> char {
        match self.items.get(&index).copied() {
            Some(SPACE) | None => LAMBDA,
            Some(c) => c,
        }
    }

    /// Read-only access to the underlying map (needed by UI for rendering).
    pub fn items(&self) -> &HashMap<i32, char> {
        &self.items
    }

    /// Formats a range of tape cells into a human-readable string.
    ///
    /// - `space_to_lambda` — replace SPACE with λ for display
    /// - `head_position`   — if `Some(pos)`, wraps that cell in parentheses
    ///
    /// Example: `[ λ, (0), 1, λ >`
    pub fn format_tape(
        &self,
        range: RangeInclusive<i32>,
        space_to_lambda: bool,
        head_position: Option<i32>,
    ) -> String {
        let mut result = String::from("[ ");

        for (i, pos) in range.enumerate() {
            if i > 0 {
                result.push_str(", ");
            }

            let ch = self.read_or_lambda(pos);
            let display_ch = if space_to_lambda && ch == SPACE {
                LAMBDA
            } else {
                ch
            };

            if head_position == Some(pos) {
                result.push('(');
                result.push(display_ch);
                result.push(')');
            } else {
                result.push(display_ch);
            }
        }

        result.push_str(" >");
        result
    }
}

impl Default for Tape {
    fn default() -> Self {
        Self::new()
    }
}
