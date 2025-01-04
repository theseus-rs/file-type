use serde::{Deserialize, Serialize};

/// The type of signature.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum SignatureType {
    #[default]
    #[serde(rename = "File extension")]
    FileExtension,
}

/// An external signature.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct ExternalSignature {
    #[serde(rename = "ExternalSignatureID")]
    id: usize,
    signature: String,
    #[serde(rename = "SignatureType")]
    r#type: SignatureType,
}

impl ExternalSignature {
    /// Create a new external signature.
    pub fn new<S: AsRef<str>>(id: usize, signature: S, r#type: SignatureType) -> Self {
        Self {
            id,
            signature: signature.as_ref().to_string(),
            r#type,
        }
    }

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
    pub fn r#type(&self) -> &SignatureType {
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
        assert!(matches!(
            external_signature.r#type(),
            SignatureType::FileExtension
        ));
        assert_eq!(external_signature.signature(), "json");
        Ok(())
    }

    #[test]
    fn test_new() {
        let external_signature = ExternalSignature::new(2421, "json", SignatureType::FileExtension);
        assert_eq!(external_signature.id(), 2421);
        assert!(matches!(
            external_signature.r#type(),
            SignatureType::FileExtension
        ));
        assert_eq!(external_signature.signature(), "json");
    }
}
