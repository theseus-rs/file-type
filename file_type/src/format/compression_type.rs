use crate::format::DocumentIdentifier;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};

/// The lossiness of a compression type
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum Lossiness {
    Lossy,
    #[default]
    Lossless,
}

/// The compression type for a file format
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct CompressionType {
    #[serde(rename = "CompressionID")]
    id: usize,
    #[serde(rename = "CompressionName")]
    name: String,
    #[serde(
        skip_serializing_if = "String::is_empty",
        rename = "CompressionAliases"
    )]
    aliases: String,
    #[serde(
        skip_serializing_if = "String::is_empty",
        rename = "CompressionFamilies"
    )]
    families: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    description: String,
    lossiness: Lossiness,
    #[serde(
        skip_serializing_if = "String::is_empty",
        rename = "CompressionDocumentation"
    )]
    documentation: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "CompressionIpr")]
    ipr: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "CompressionNote")]
    note: String,
    #[serde(rename = "CompressionIdentifier")]
    identifiers: Vec<DocumentIdentifier>,
}

impl CompressionType {
    /// Create a new compression type
    #[expect(clippy::too_many_arguments)]
    pub fn new<S: AsRef<str>>(
        id: usize,
        name: S,
        aliases: S,
        families: S,
        description: S,
        lossiness: Lossiness,
        documentation: S,
        ipr: S,
        note: S,
        identifiers: Vec<DocumentIdentifier>,
    ) -> Self {
        Self {
            id,
            name: name.as_ref().to_string(),
            aliases: aliases.as_ref().to_string(),
            families: families.as_ref().to_string(),
            description: description.as_ref().to_string(),
            lossiness,
            documentation: documentation.as_ref().to_string(),
            ipr: ipr.as_ref().to_string(),
            note: note.as_ref().to_string(),
            identifiers,
        }
    }

    /// Get the compression ID
    #[must_use]
    pub fn id(&self) -> usize {
        self.id
    }

    /// Get the compression name
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the compression aliases
    #[must_use]
    pub fn aliases(&self) -> &str {
        &self.aliases
    }

    /// Get the compression families
    #[must_use]
    pub fn families(&self) -> &str {
        &self.families
    }

    /// Get the compression description
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get the compression lossiness
    #[must_use]
    pub fn lossiness(&self) -> &Lossiness {
        &self.lossiness
    }

    /// Get the compression documentation
    #[must_use]
    pub fn documentation(&self) -> &str {
        &self.documentation
    }

    /// Get the compression IPR
    #[must_use]
    pub fn ipr(&self) -> &str {
        &self.ipr
    }

    /// Get the compression note
    #[must_use]
    pub fn note(&self) -> &str {
        &self.note
    }

    /// Get the compression identifiers
    #[must_use]
    pub fn identifiers(&self) -> &[DocumentIdentifier] {
        &self.identifiers
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

        assert_eq!(compression_types.id(), 3);
        assert_eq!(compression_types.name(), "Pulse Code Modulation");
        assert_eq!(compression_types.aliases(), "");
        assert_eq!(compression_types.families(), "");
        assert_eq!(compression_types.description(), "Uncompressed audio encoding method, which uses linear sampling at a wide range of sampling frequencies and resolutions.");
        assert!(matches!(compression_types.lossiness(), Lossiness::Lossless));
        assert_eq!(compression_types.documentation(), "");
        assert_eq!(compression_types.ipr(), "");
        assert_eq!(compression_types.note(), "");

        let identifiers = compression_types.identifiers();
        assert_eq!(identifiers.len(), 1);
        let identifier = &identifiers[0];
        assert_eq!(identifier.identifier(), "x-cmp/3");
        assert_eq!(identifier.r#type(), "PUID");
        Ok(())
    }

    #[test]
    fn test_new() {
        let compression_types = CompressionType::new(
            3,
            "Pulse Code Modulation",
            "",
            "",
            "Uncompressed audio encoding method, which uses linear sampling at a wide range of sampling frequencies and resolutions.",
            Lossiness::Lossless,
            "",
            "",
            "",
            vec![DocumentIdentifier::new("x-cmp/3", "PUID")],
        );
        assert_eq!(compression_types.id(), 3);
        assert_eq!(compression_types.name(), "Pulse Code Modulation");
        assert_eq!(compression_types.aliases(), "");
        assert_eq!(compression_types.families(), "");
        assert_eq!(compression_types.description(), "Uncompressed audio encoding method, which uses linear sampling at a wide range of sampling frequencies and resolutions.");
        assert!(matches!(compression_types.lossiness(), Lossiness::Lossless));
        assert_eq!(compression_types.documentation(), "");
        assert_eq!(compression_types.ipr(), "");
        assert_eq!(compression_types.note(), "");

        let identifiers = compression_types.identifiers();
        assert_eq!(identifiers.len(), 1);
        let identifier = &identifiers[0];
        assert_eq!(identifier.identifier(), "x-cmp/3");
        assert_eq!(identifier.r#type(), "PUID");
    }
}
