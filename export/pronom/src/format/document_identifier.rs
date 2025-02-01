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
    use indoc::indoc;
    use quick_xml::de::from_str;
    use quick_xml::se::to_string;

    #[test]
    fn test_serde() -> anyhow::Result<()> {
        let xml = indoc! {r"
          <DocumentIdentifier>
            <Identifier>application/json</Identifier>
            <IdentifierType>MIME</IdentifierType>
          </DocumentIdentifier>
        "};
        let document_identifier: DocumentIdentifier = from_str(xml)?;

        // Test serialization
        let xml = to_string(&document_identifier)?;
        let document_identifier: DocumentIdentifier = from_str(xml.as_str())?;

        assert_eq!(document_identifier.identifier, "application/json");
        assert_eq!(document_identifier.r#type, "MIME");
        Ok(())
    }
}
