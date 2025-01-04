use crate::pronom::external_signature::ExternalSignature;
use crate::pronom::ByteSequence;
use serde::{Deserialize, Serialize};

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
    #[serde(rename = "ByteSequence")]
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

    /// Check if this internal signature is a match for the given bytes
    #[must_use]
    pub fn is_match(&self, bytes: &[u8]) -> bool {
        // All byte sequences must match in order for the internal signature to match
        self.byte_sequences
            .iter()
            .all(|byte_sequence| byte_sequence.is_match(bytes))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pronom::ByteSequence;
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
