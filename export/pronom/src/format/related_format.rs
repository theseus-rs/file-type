use serde::{Deserialize, Serialize};

/// The type of relationship between two format
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) enum RelationshipType {
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
pub(crate) struct RelatedFormat {
    #[serde(rename = "RelationshipType")]
    pub(crate) relationship_type: RelationshipType,
    #[serde(rename = "RelatedFormatID")]
    pub(crate) id: usize,
    #[serde(rename = "RelatedFormatName")]
    pub(crate) name: String,
}

impl RelatedFormat {
    /// Convert to the type used at runtime
    #[expect(clippy::unnecessary_wraps)]
    pub fn to_runtime_type(&self) -> file_type::Result<file_type::format::RelatedFormat> {
        let relationship_type = match self.relationship_type {
            RelationshipType::CanContain => file_type::format::RelationshipType::CanContain,
            RelationshipType::CanBeContainedBy => {
                file_type::format::RelationshipType::CanBeContainedBy
            }
            RelationshipType::EquivalentTo => file_type::format::RelationshipType::EquivalentTo,
            RelationshipType::HasPriorityOver => {
                file_type::format::RelationshipType::HasPriorityOver
            }
            RelationshipType::HasLowerPriorityThan => {
                file_type::format::RelationshipType::HasLowerPriorityThan
            }
            RelationshipType::IsPreviousVersionOf => {
                file_type::format::RelationshipType::IsPreviousVersionOf
            }
            RelationshipType::IsSubsequentVersionOf => {
                file_type::format::RelationshipType::IsSubsequentVersionOf
            }
            RelationshipType::IsSubtypeOf => file_type::format::RelationshipType::IsSubtypeOf,
            RelationshipType::IsSupertypeOf => file_type::format::RelationshipType::IsSupertypeOf,
        };
        let relates_format = file_type::format::RelatedFormat {
            id: self.id,
            relationship_type,
        };
        Ok(relates_format)
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
            related_format.relationship_type,
            RelationshipType::HasPriorityOver
        ));
        assert_eq!(related_format.id, 1617);
        assert_eq!(related_format.name, "JSON Data Interchange Format");
        Ok(())
    }
}
