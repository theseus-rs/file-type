use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Publisher {
    #[serde(rename = "PublisherID")]
    id: usize,
    #[serde(rename = "PublisherName")]
    name: String,
    organisation_name: String,
    #[serde(rename = "PublisherCompoundName")]
    compound_name: String,
}

impl Publisher {
    /// Create a new publisher
    pub fn new<S: AsRef<str>>(id: usize, name: S, organisation_name: S, compound_name: S) -> Self {
        Self {
            id,
            name: name.as_ref().to_string(),
            organisation_name: organisation_name.as_ref().to_string(),
            compound_name: compound_name.as_ref().to_string(),
        }
    }

    /// Get the publisher ID
    #[must_use]
    pub fn id(&self) -> usize {
        self.id
    }

    /// Get the publisher name
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the organisation name
    #[must_use]
    pub fn organisation_name(&self) -> &str {
        &self.organisation_name
    }

    /// Get the compound name
    #[must_use]
    pub fn compound_name(&self) -> &str {
        &self.compound_name
    }
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
          <Publisher>
            <PublisherID>1</PublisherID>
            <PublisherName>Publisher</PublisherName>
            <OrganisationName>Organization</OrganisationName>
            <PublisherCompoundName>Compound</PublisherCompoundName>
          </Publisher>
        "};
        let publisher: Publisher = from_str(xml)?;

        // Test serialization
        let xml = to_string(&publisher)?;
        let publisher: Publisher = from_str(xml.as_str())?;

        assert_eq!(publisher.id(), 1);
        assert_eq!(publisher.name(), "Publisher");
        assert_eq!(publisher.organisation_name(), "Organization");
        assert_eq!(publisher.compound_name(), "Compound");
        Ok(())
    }

    #[test]
    fn test_new() {
        let publisher = Publisher::new(1, "Publisher", "Organization", "Compound");
        assert_eq!(publisher.id(), 1);
        assert_eq!(publisher.name(), "Publisher");
        assert_eq!(publisher.organisation_name(), "Organization");
        assert_eq!(publisher.compound_name(), "Compound");
    }
}
