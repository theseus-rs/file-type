use crate::format::{ByteSequence, PositionType};

/// A file signature.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Signature {
    pub byte_sequences: &'static [ByteSequence],
}

impl Signature {
    /// Get the key for this signature.
    ///
    /// The key is the first 8 bytes on the first byte sequence that is an absolute position from
    /// the beginning of the file where the sequence is a literal.  If no such byte sequence exists,
    /// the key is 0.
    #[must_use]
    pub fn key(&self) -> u64 {
        for byte_sequence in self.byte_sequences {
            if byte_sequence.position_type == PositionType::BOF {
                if let Some(0) = byte_sequence.offset {
                    let key = byte_sequence.regex.key();
                    if key != 0 {
                        return key;
                    }
                }
            }
        }
        match self.byte_sequences.first() {
            Some(byte_sequence) => {
                if !matches!(byte_sequence.position_type, PositionType::BOF) {
                    return 0;
                }
                byte_sequence.regex.key()
            }
            None => 0,
        }
    }

    /// Check if this signature is a match for the given bytes
    #[must_use]
    pub fn is_match(&self, bytes: &[u8]) -> bool {
        // All byte sequences must match in order for the signature to match
        self.byte_sequences
            .iter()
            .all(|byte_sequence| byte_sequence.is_match(bytes))
    }
}
