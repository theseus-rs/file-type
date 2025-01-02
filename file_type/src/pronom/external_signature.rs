use serde::{Deserialize, Serialize};

/// An external signature.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ExternalSignature {
    #[serde(rename = "ExternalSignatureID")]
    id: usize,
    signature: String,
    #[serde(rename = "SignatureType")]
    r#type: String,
}

impl ExternalSignature {
    /// Get the ID of the external signature.
    #[must_use]
    pub fn id(&self) -> usize {
        self.id
    }

    /// Get the signature.
    #[must_use]
    pub fn signature(&self) -> &str {
        &self.signature
    }

    /// Get the type of the signature.
    #[must_use]
    pub fn r#type(&self) -> &str {
        &self.r#type
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
          <ExternalSignature>
            <ExternalSignatureID>2421</ExternalSignatureID>
            <Signature>json</Signature>
            <SignatureType>File extension</SignatureType>
          </ExternalSignature>
        "};
        let external_signature: ExternalSignature = from_str(xml)?;

        // Test serialization
        let xml = to_string(&external_signature)?;
        let external_signature: ExternalSignature = from_str(xml.as_str())?;

        assert_eq!(external_signature.id(), 2421);
        assert_eq!(external_signature.signature(), "json");
        assert_eq!(external_signature.r#type(), "File extension");
        Ok(())
    }
}
