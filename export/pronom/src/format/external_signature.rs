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
    use crate::format::test_utils::round_trip;
    use indoc::indoc;

    #[test]
    fn test_serde() {
        let xml = indoc! {r"
          <ExternalSignature>
            <ExternalSignatureID>2421</ExternalSignatureID>
            <Signature>json</Signature>
            <SignatureType>File extension</SignatureType>
          </ExternalSignature>
        "};
        let external_signature: anyhow::Result<ExternalSignature> = round_trip(xml);
        assert!(external_signature.is_ok());
        let external_signature = external_signature.unwrap();

        assert_eq!(external_signature.id, 2421);
        assert!(matches!(
            external_signature.r#type,
            SignatureType::FileExtension
        ));
        assert_eq!(external_signature.signature, "json");
    }
}
