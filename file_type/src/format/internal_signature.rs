use crate::format::external_signature::ExternalSignature;
use crate::format::{ByteSequence, PositionType};
use serde::{Deserialize, Deserializer, Serialize};

/// An internal signature.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct InternalSignature {
    #[serde(rename = "SignatureID")]
    id: usize,
    #[serde(rename = "SignatureName")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "SignatureNote")]
    note: String,
    #[serde(
        rename = "ByteSequence",
        deserialize_with = "deserialize_and_sort_byte_sequences"
    )]
    byte_sequences: Vec<ByteSequence>,
}

impl InternalSignature {
    /// Create a new internal signature.
    pub fn new<S: AsRef<str>>(
        id: usize,
        name: S,
        note: S,
        byte_sequences: Vec<ByteSequence>,
    ) -> Self {
        Self {
            id,
            name: name.as_ref().to_string(),
            note: note.as_ref().to_string(),
            byte_sequences,
        }
    }

    /// Get the ID of the internal signature.
    #[must_use]
    pub fn id(&self) -> usize {
        self.id
    }

    /// Get the name of the internal signature.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the note of the internal signature.
    #[must_use]
    pub fn note(&self) -> &str {
        &self.note
    }

    /// Get the byte sequences of the internal signature.
    #[must_use]
    pub fn byte_sequences(&self) -> &[ByteSequence] {
        &self.byte_sequences
    }

    /// Get the key for this internal signature.
    ///
    /// The key is the first 8 bytes on the first byte sequence that is an absolute position from
    /// the beginning of the file where the sequence is a literal.  If no such byte sequence exists,
    /// the key is 0.
    #[must_use]
    pub fn key(&self) -> u64 {
        for byte_sequence in &self.byte_sequences {
            match byte_sequence.position_type() {
                PositionType::AbsoluteFromBOF => {
                    if let Some(0) = byte_sequence.offset() {
                        let key = byte_sequence.regex().key();
                        if key != 0 {
                            return key;
                        }
                    }
                }
                _ => continue,
            }
        }
        match self.byte_sequences.first() {
            Some(byte_sequence) => {
                if !matches!(byte_sequence.position_type(), PositionType::AbsoluteFromBOF) {
                    return 0;
                }
                byte_sequence.regex().key()
            }
            None => 0,
        }
    }

    /// Check if this internal signature is a match for the given bytes
    #[must_use]
    pub fn is_match(&self, bytes: &[u8]) -> bool {
        // All byte sequences must match in order for the internal signature to match
        self.byte_sequences
            .iter()
            .all(|byte_sequence| byte_sequence.is_match(bytes))
    }
}

fn deserialize_and_sort_byte_sequences<'de, D>(
    deserializer: D,
) -> Result<Vec<ByteSequence>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut byte_sequences: Vec<ByteSequence> = Vec::deserialize(deserializer)?;
    // Sort byte sequences by position type such that Variable comes last.  All current uses of
    // Variable byte sequences also include a BOF/EOF sequence.  This is an optimization
    // to check BOF/EOF sequences first to avoid checking Variable sequences when possible.
    byte_sequences.sort_by_key(|byte_sequence| match byte_sequence.position_type() {
        PositionType::AbsoluteFromBOF => 0,
        PositionType::AbsoluteFromEOF => 1,
        PositionType::Variable => 2,
    });
    Ok(byte_sequences)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::format::ByteSequence;
    use anyhow::Result;
    use indoc::indoc;
    use quick_xml::de::from_str;
    use quick_xml::se::to_string;

    #[test]
    fn test_serde() -> Result<()> {
        let xml = indoc! {r"
          <InternalSignature>
            <SignatureID>1687</SignatureID>
            <SignatureName>Tweet JSON (Raw JSON)</SignatureName>
            <SignatureNote>The signature assumes a starting { character</SignatureNote>
          </InternalSignature>
        "};
        let internal_signature: InternalSignature = from_str(xml)?;

        // Test serialization
        let xml = to_string(&internal_signature)?;
        let internal_signature: InternalSignature = from_str(xml.as_str())?;

        assert_eq!(internal_signature.id(), 1687);
        assert_eq!(internal_signature.name(), "Tweet JSON (Raw JSON)");
        assert_eq!(
            internal_signature.note(),
            "The signature assumes a starting { character"
        );

        let byte_sequences = internal_signature.byte_sequences();
        assert_eq!(byte_sequences.len(), 0);
        Ok(())
    }

    #[test]
    fn test_new() {
        let internal_signature = InternalSignature::new(
            1687,
            "Tweet JSON (Raw JSON)",
            "The signature assumes a starting { character",
            vec![],
        );
        assert_eq!(internal_signature.id(), 1687);
        assert_eq!(internal_signature.name(), "Tweet JSON (Raw JSON)");
        assert_eq!(
            internal_signature.note(),
            "The signature assumes a starting { character"
        );
        assert_eq!(internal_signature.byte_sequences().len(), 0);
    }
}
