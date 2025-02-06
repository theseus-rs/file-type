use crate::format::regex::Regex;
use core::str::FromStr;

/// The position type for a byte sequence
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum PositionType {
    #[default]
    BOF,
    EOF,
    Variable,
}

/// A byte sequence used to identify a file format
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ByteSequence {
    pub position_type: PositionType,
    pub offset: Option<usize>,
    pub regex: Regex,
}

impl ByteSequence {
    /// Check if the given data matches the byte sequence
    #[must_use]
    pub fn is_match(&self, bytes: &[u8]) -> bool {
        match self.position_type {
            PositionType::BOF => {
                let offset = self.offset.unwrap_or_default();
                self.regex.is_match_at(bytes, offset)
            }
            PositionType::EOF => {
                let offset = self.offset.unwrap_or_default();
                let value = self.regex.to_string();
                let regex_len = value.len() / 2;
                let offset = usize::checked_add(regex_len, offset).unwrap_or(regex_len);
                let offset = usize::checked_sub(bytes.len(), offset).unwrap_or(bytes.len());
                self.regex.is_match_at(bytes, offset)
            }
            PositionType::Variable => {
                for offset in self.offset.unwrap_or_default()..bytes.len() {
                    if self.regex.is_match_at(bytes, offset) {
                        return true;
                    }
                }
                false
            }
        }
    }
}
