use serde::{Deserialize, Serialize};

/// The type of signature.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) enum SignatureType {
    #[default]
    #[serde(rename = "File extension")]
    FileExtension,
}

/// An external signature.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub(crate) struct ExternalSignature {
    #[serde(rename = "ExternalSignatureID")]
    pub(crate) id: usize,
    pub(crate) signature: String,
    #[serde(rename = "SignatureType")]
    pub(crate) r#type: SignatureType,
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

        assert_eq!(external_signature.id, 2421);
        assert!(matches!(
            external_signature.r#type,
            SignatureType::FileExtension
        ));
        assert_eq!(external_signature.signature, "json");
        Ok(())
    }
}
