use serde::{Deserialize, Serialize};

/// A document identifier
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub(crate) struct DocumentIdentifier {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) identifier: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "IdentifierType")]
    pub(crate) r#type: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::format::test_utils::round_trip;
    use indoc::indoc;

    #[test]
    fn test_serde() {
        let xml = indoc! {r"
          <DocumentIdentifier>
            <Identifier>application/json</Identifier>
            <IdentifierType>MIME</IdentifierType>
          </DocumentIdentifier>
        "};
        let document_identifier: anyhow::Result<DocumentIdentifier> = round_trip(xml);
        assert!(document_identifier.is_ok());
        let document_identifier = document_identifier.unwrap();

        assert_eq!(document_identifier.identifier, "application/json");
        assert_eq!(document_identifier.r#type, "MIME");
    }
}
