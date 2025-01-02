use serde::{Deserialize, Serialize};

/// The type of relationship between two formats
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum RelationshipType {
    #[serde(rename = "Can contain")]
    CanContain,
    #[serde(rename = "Can be contained by")]
    CanBeContainedBy,
    #[default]
    #[serde(rename = "Equivalent to")]
    EquivalentTo,
    #[serde(rename = "Has priority over")]
    HasPriorityOver,
    #[serde(rename = "Has lower priority than")]
    HasLowerPriorityThan,
    #[serde(rename = "Is previous version of")]
    IsPreviousVersionOf,
    #[serde(rename = "Is subsequent version of")]
    IsSubsequentVersionOf,
    #[serde(rename = "Is subtype of")]
    IsSubtypeOf,
    #[serde(rename = "Is supertype of")]
    IsSupertypeOf,
}

/// A related format
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct RelatedFormat {
    #[serde(rename = "RelationshipType")]
    relationship_type: RelationshipType,
    #[serde(rename = "RelatedFormatID")]
    id: usize,
    #[serde(rename = "RelatedFormatName")]
    name: String,
    #[serde(rename = "RelatedFormatVersion")]
    version: String,
}

impl RelatedFormat {
    /// Get the relationship type
    #[must_use]
    pub fn relationship_type(&self) -> &RelationshipType {
        &self.relationship_type
    }

    /// Get the related id
    #[must_use]
    pub fn id(&self) -> usize {
        self.id
    }

    /// Get the related name
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the related version
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
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
          <RelatedFormat>
            <RelationshipType>Has priority over</RelationshipType>
            <RelatedFormatID>1617</RelatedFormatID>
            <RelatedFormatName>JSON Data Interchange Format</RelatedFormatName>
            <RelatedFormatVersion>
            </RelatedFormatVersion>
          </RelatedFormat>
        "};
        let related_format: RelatedFormat = from_str(xml)?;

        // Test serialization
        let xml = to_string(&related_format)?;
        let related_format: RelatedFormat = from_str(xml.as_str())?;

        assert!(matches!(
            related_format.relationship_type(),
            RelationshipType::HasPriorityOver
        ));
        assert_eq!(related_format.id(), 1617);
        assert_eq!(related_format.name(), "JSON Data Interchange Format");
        assert_eq!(related_format.version(), "");
        Ok(())
    }
}
