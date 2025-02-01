use crate::format::byte_sequence::{ByteSequence, PositionType};
use serde::{Deserialize, Deserializer, Serialize};

/// An internal signature.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub(crate) struct InternalSignature {
    #[serde(rename = "SignatureID")]
    pub(crate) id: usize,
    #[serde(rename = "SignatureName")]
    pub(crate) name: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "SignatureNote")]
    pub(crate) note: String,
    #[serde(
        rename = "ByteSequence",
        deserialize_with = "deserialize_and_sort_byte_sequences"
    )]
    pub(crate) byte_sequences: Vec<ByteSequence>,
}

impl InternalSignature {
    /// Convert to the type used at runtime
    pub fn to_runtime_type(&self) -> file_type::Result<file_type::format::InternalSignature> {
        let mut byte_sequences = Vec::new();
        for byte_sequence in &self.byte_sequences {
            byte_sequences.push(byte_sequence.to_runtime_type()?);
        }
        let internal_signature = file_type::format::InternalSignature {
            byte_sequences: Box::leak(byte_sequences.into_boxed_slice()),
        };
        Ok(internal_signature)
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
    byte_sequences.sort_by_key(|byte_sequence| match byte_sequence.position_type {
        PositionType::AbsoluteFromBOF => 0,
        PositionType::AbsoluteFromEOF => 1,
        PositionType::Variable => 2,
    });
    Ok(byte_sequences)
}

#[cfg(test)]
mod test {
    use super::*;
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

        assert_eq!(internal_signature.id, 1687);
        assert_eq!(internal_signature.name, "Tweet JSON (Raw JSON)");
        assert_eq!(
            internal_signature.note,
            "The signature assumes a starting { character"
        );

        let byte_sequences = internal_signature.byte_sequences;
        assert_eq!(byte_sequences.len(), 0);
        Ok(())
    }
}
