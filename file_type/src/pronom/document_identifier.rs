use serde::{Deserialize, Serialize};

/// A document identifier
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct DocumentIdentifier {
    identifier: String,
    #[serde(rename = "IdentifierType")]
    r#type: String,
}

impl DocumentIdentifier {
    /// Create a new document identifier
    pub fn new<S: AsRef<str>>(identifier: S, r#type: S) -> Self {
        Self {
            identifier: identifier.as_ref().to_string(),
            r#type: r#type.as_ref().to_string(),
        }
    }

    /// Get the identifier
    #[must_use]
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    /// Get the identifier type
    #[must_use]
    pub fn r#type(&self) -> &str {
        &self.r#type
    }
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
        assert_eq!(document_identifier.r#type(), "MIME");
        Ok(())
    }

    #[test]
    fn test_new() {
        let document_identifier = DocumentIdentifier::new("application/json", "MIME");
        assert_eq!(document_identifier.identifier, "application/json");
        assert_eq!(document_identifier.r#type, "MIME");
    }
}
