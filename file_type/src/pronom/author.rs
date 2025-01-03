use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct Author {
    #[serde(rename = "AuthorID")]
    id: usize,
    #[serde(skip_serializing_if = "String::is_empty", rename = "AuthorName")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    organisation_name: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "AuthorCompoundName")]
    compound_name: String,
}

impl Author {
    /// Create a new author
    pub fn new<S: AsRef<str>>(id: usize, name: S, organisation_name: S, compound_name: S) -> Self {
        Self {
            id,
            name: name.as_ref().to_string(),
            organisation_name: organisation_name.as_ref().to_string(),
            compound_name: compound_name.as_ref().to_string(),
        }
    }

    /// Get the author ID
    #[must_use]
    pub fn id(&self) -> usize {
        self.id
    }

    /// Get the author name
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
          <Author>
            <AuthorID>1</AuthorID>
            <AuthorName>Author</AuthorName>
            <OrganisationName>Organization</OrganisationName>
            <AuthorCompoundName>Compound</AuthorCompoundName>
          </Author>
        "};
        let author: Author = from_str(xml)?;

        // Test serialization
        let xml = to_string(&author)?;
        let author: Author = from_str(xml.as_str())?;

        assert_eq!(author.id(), 1);
        assert_eq!(author.name(), "Author");
        assert_eq!(author.organisation_name(), "Organization");
        assert_eq!(author.compound_name(), "Compound");
        Ok(())
    }

    #[test]
    fn test_new() {
        let author = Author::new(1, "Author", "Organization", "Compound");
        assert_eq!(author.id(), 1);
        assert_eq!(author.name(), "Author");
        assert_eq!(author.organisation_name(), "Organization");
        assert_eq!(author.compound_name(), "Compound");
    }
}
