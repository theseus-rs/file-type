use crate::format::document_identifier::DocumentIdentifier;
use serde::{Deserialize, Serialize};

/// Custom deserializer that trims whitespace and converts whitespace-only strings to empty strings
fn deserialize_trimmed_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.trim().to_string())
}

/// The lossiness of a compression type
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) enum Lossiness {
    Lossy,
    #[default]
    Lossless,
}

/// The compression type for a file format
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub(crate) struct CompressionType {
    #[serde(rename = "CompressionID")]
    pub(crate) id: usize,
    #[serde(
        rename = "CompressionName",
        deserialize_with = "deserialize_trimmed_string"
    )]
    pub(crate) name: String,
    #[serde(
        skip_serializing_if = "String::is_empty",
        rename = "CompressionAliases",
        deserialize_with = "deserialize_trimmed_string"
    )]
    pub(crate) aliases: String,
    #[serde(
        skip_serializing_if = "String::is_empty",
        rename = "CompressionFamilies",
        deserialize_with = "deserialize_trimmed_string"
    )]
    pub(crate) families: String,
    #[serde(
        skip_serializing_if = "String::is_empty",
        deserialize_with = "deserialize_trimmed_string"
    )]
    pub(crate) description: String,
    pub(crate) lossiness: Lossiness,
    #[serde(
        skip_serializing_if = "String::is_empty",
        rename = "CompressionDocumentation",
        deserialize_with = "deserialize_trimmed_string"
    )]
    pub(crate) documentation: String,
    #[serde(
        skip_serializing_if = "String::is_empty",
        rename = "CompressionIpr",
        deserialize_with = "deserialize_trimmed_string"
    )]
    pub(crate) ipr: String,
    #[serde(
        skip_serializing_if = "String::is_empty",
        rename = "CompressionNote",
        deserialize_with = "deserialize_trimmed_string"
    )]
    pub(crate) note: String,
    #[serde(rename = "CompressionIdentifier")]
    pub(crate) identifiers: Vec<DocumentIdentifier>,
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
          <CompressionType>
            <CompressionID>3</CompressionID>
            <CompressionName>Pulse Code Modulation</CompressionName>
            <CompressionVersion>
            </CompressionVersion>
            <CompressionAliases>
            </CompressionAliases>
            <CompressionFamilies>
            </CompressionFamilies>
            <Description>Uncompressed audio encoding method, which uses linear sampling at a wide range of sampling frequencies and resolutions.</Description>
            <Lossiness>Lossless</Lossiness>
            <ReleaseDate>04 Jul 2001</ReleaseDate>
            <WithdrawnDate>
            </WithdrawnDate>
            <CompressionDocumentation>
            </CompressionDocumentation>
            <CompressionIPR>
            </CompressionIPR>
            <CompressionNote>
            </CompressionNote>
            <CompressionIdentifier>
              <Identifier>x-cmp/3</Identifier>
              <IdentifierType>PUID</IdentifierType>
            </CompressionIdentifier>
          </CompressionType>
        "};
        let compression_types: CompressionType = from_str(xml)?;

        // Test serialization
        let xml = to_string(&compression_types)?;
        let compression_types: CompressionType = from_str(xml.as_str())?;

        assert_eq!(compression_types.id, 3);
        assert_eq!(compression_types.name, "Pulse Code Modulation");
        assert_eq!(compression_types.aliases, "");
        assert_eq!(compression_types.families, "");
        assert_eq!(
            compression_types.description,
            "Uncompressed audio encoding method, which uses linear sampling at a wide range of sampling frequencies and resolutions."
        );
        assert!(matches!(compression_types.lossiness, Lossiness::Lossless));
        assert_eq!(compression_types.documentation, "");
        assert_eq!(compression_types.ipr, "");
        assert_eq!(compression_types.note, "");

        let identifiers = compression_types.identifiers;
        assert_eq!(identifiers.len(), 1);
        let identifier = &identifiers[0];
        assert_eq!(identifier.identifier, "x-cmp/3");
        assert_eq!(identifier.r#type, "PUID");
        Ok(())
    }
}
