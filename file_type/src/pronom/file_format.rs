use crate::pronom::serde::{
    deserialize_naive_date, deserialize_option_naive_date, serialize_naive_date,
    serialize_option_naive_date,
};
use crate::pronom::{
    CompressionType, Document, DocumentIdentifier, ExternalSignature, InternalSignature,
    RelatedFormat,
};
use jiff::civil::Date;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub enum FormatTypes {
    Aggregate,
    Audio,
    Database,
    #[default]
    Dataset,
    Font,
    #[serde(rename = "Image (Raster)")]
    ImageRaster,
    #[serde(rename = "Image (Vector)")]
    ImageVector,
    Presentation,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub struct FileFormat {
    #[serde(rename = "FormatID")]
    id: usize,
    #[serde(rename = "FormatName")]
    name: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "FormatVersion")]
    version: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "FormatAliases")]
    aliases: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "FormatFamilies")]
    families: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "FormatTypes")]
    types: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "FormatDisclosure")]
    disclosure: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "FormatDescription")]
    description: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    binary_file_format: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    byte_orders: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_naive_date",
        serialize_with = "serialize_option_naive_date",
    )]
    release_date: Option<Date>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_option_naive_date",
        serialize_with = "serialize_option_naive_date",
    )]
    withdrawn_date: Option<Date>,
    provenance_source_id: usize,
    #[serde(skip_serializing_if = "String::is_empty")]
    provenance_name: String,
    #[serde(
        deserialize_with = "deserialize_naive_date",
        serialize_with = "serialize_naive_date"
    )]
    provenance_source_date: Date,
    #[serde(skip_serializing_if = "String::is_empty")]
    provenance_description: String,
    #[serde(
        deserialize_with = "deserialize_naive_date",
        serialize_with = "serialize_naive_date"
    )]
    last_updated_date: Date,
    #[serde(skip_serializing_if = "String::is_empty", rename = "FormatNote")]
    note: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "FormatRisk")]
    risk: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    technical_environment: String,
    #[serde(rename = "FileFormatIdentifier")]
    file_format_identifiers: Vec<DocumentIdentifier>,
    #[serde(rename = "Document")]
    documents: Vec<Document>,
    #[serde(rename = "ExternalSignature")]
    external_signatures: Vec<ExternalSignature>,
    #[serde(rename = "InternalSignature")]
    internal_signatures: Vec<InternalSignature>,
    #[serde(rename = "RelatedFormat")]
    related_formats: Vec<RelatedFormat>,
    #[serde(rename = "CompressionType")]
    compression_types: Vec<CompressionType>,
}

impl FileFormat {
    /// Create a new file format
    #[expect(clippy::too_many_arguments)]
    pub fn new<S: AsRef<str>>(
        id: usize,
        name: S,
        version: S,
        aliases: S,
        families: S,
        types: S,
        disclosure: S,
        description: S,
        binary_file_format: S,
        byte_orders: S,
        release_date: Option<Date>,
        withdrawn_date: Option<Date>,
        provenance_source_id: usize,
        provenance_name: S,
        provenance_source_date: Date,
        provenance_description: S,
        last_updated_date: Date,
        note: S,
        risk: S,
        technical_environment: S,
        file_format_identifiers: Vec<DocumentIdentifier>,
        documents: Vec<Document>,
        external_signatures: Vec<ExternalSignature>,
        internal_signatures: Vec<InternalSignature>,
        related_formats: Vec<RelatedFormat>,
        compression_types: Vec<CompressionType>,
    ) -> Self {
        Self {
            id,
            name: name.as_ref().to_string(),
            version: version.as_ref().to_string(),
            aliases: aliases.as_ref().to_string(),
            families: families.as_ref().to_string(),
            types: types.as_ref().to_string(),
            disclosure: disclosure.as_ref().to_string(),
            description: description.as_ref().to_string(),
            binary_file_format: binary_file_format.as_ref().to_string(),
            byte_orders: byte_orders.as_ref().to_string(),
            release_date,
            withdrawn_date,
            provenance_source_id,
            provenance_name: provenance_name.as_ref().to_string(),
            provenance_source_date,
            provenance_description: provenance_description.as_ref().to_string(),
            last_updated_date,
            note: note.as_ref().to_string(),
            risk: risk.as_ref().to_string(),
            technical_environment: technical_environment.as_ref().to_string(),
            file_format_identifiers,
            documents,
            external_signatures,
            internal_signatures,
            related_formats,
            compression_types,
        }
    }

    /// Get the format ID
    #[must_use]
    pub fn id(&self) -> usize {
        self.id
    }

    /// Get the format PUID
    #[must_use]
    pub fn puid(&self) -> &str {
        let file_format_identifier = self
            .file_format_identifiers
            .iter()
            .find(|identifier| identifier.r#type() == "PUID");
        let Some(file_format_identifier) = file_format_identifier else {
            return "";
        };
        file_format_identifier.identifier()
    }

    /// Get the format name
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the format version
    #[must_use]
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Get the format aliases
    #[must_use]
    pub fn aliases(&self) -> &str {
        &self.aliases
    }

    /// Get the format families
    #[must_use]
    pub fn families(&self) -> &str {
        &self.families
    }

    /// Get the format types
    #[must_use]
    pub fn types(&self) -> &str {
        &self.types
    }

    /// Get the format disclosure
    #[must_use]
    pub fn disclosure(&self) -> &str {
        &self.disclosure
    }

    /// Get the format description
    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get the binary file format
    #[must_use]
    pub fn binary_file_format(&self) -> &str {
        &self.binary_file_format
    }

    /// Get the byte orders
    #[must_use]
    pub fn byte_orders(&self) -> &str {
        &self.byte_orders
    }

    /// Get the release date
    #[must_use]
    pub fn release_date(&self) -> &Option<Date> {
        &self.release_date
    }

    /// Get the withdrawn date
    #[must_use]
    pub fn withdrawn_date(&self) -> &Option<Date> {
        &self.withdrawn_date
    }

    /// Get the provenance source ID
    #[must_use]
    pub fn provenance_source_id(&self) -> usize {
        self.provenance_source_id
    }

    /// Get the provenance name
    #[must_use]
    pub fn provenance_name(&self) -> &str {
        &self.provenance_name
    }

    /// Get the provenance source date
    #[must_use]
    pub fn provenance_source_date(&self) -> &Date {
        &self.provenance_source_date
    }

    /// Get the provenance description
    #[must_use]
    pub fn provenance_description(&self) -> &str {
        &self.provenance_description
    }

    /// Get the last updated date
    #[must_use]
    pub fn last_updated_date(&self) -> &Date {
        &self.last_updated_date
    }

    /// Get the format note
    #[must_use]
    pub fn note(&self) -> &str {
        &self.note
    }

    /// Get the format risk
    #[must_use]
    pub fn risk(&self) -> &str {
        &self.risk
    }

    /// Get the technical environment
    #[must_use]
    pub fn technical_environment(&self) -> &str {
        &self.technical_environment
    }

    /// Get the file format identifiers
    #[must_use]
    pub fn file_format_identifiers(&self) -> &[DocumentIdentifier] {
        &self.file_format_identifiers
    }

    /// Get the documents
    #[must_use]
    pub fn documents(&self) -> &[Document] {
        &self.documents
    }

    /// Get the external signatures
    #[must_use]
    pub fn external_signatures(&self) -> &[ExternalSignature] {
        &self.external_signatures
    }

    /// Get the internal signatures
    #[must_use]
    pub fn internal_signatures(&self) -> &[InternalSignature] {
        &self.internal_signatures
    }

    /// Get the related formats
    #[must_use]
    pub fn related_formats(&self) -> &[RelatedFormat] {
        &self.related_formats
    }

    /// Get the compression types
    #[must_use]
    pub fn compression_types(&self) -> &[CompressionType] {
        &self.compression_types
    }

    /// Get extensions
    #[must_use]
    pub fn extensions(&self) -> Vec<&str> {
        self.external_signatures
            .iter()
            .map(ExternalSignature::signature)
            .collect()
    }

    /// Get media types
    #[must_use]
    pub fn media_types(&self) -> Vec<&str> {
        self.file_format_identifiers
            .iter()
            .filter_map(|identifier| {
                if identifier.r#type() == "MIME" {
                    Some(identifier.identifier())
                } else {
                    None
                }
            })
            .collect()
    }

    /// Check if this file format is a match for the given bytes
    #[must_use]
    pub fn is_match(&self, bytes: &[u8]) -> bool {
        self.internal_signatures()
            .iter()
            .any(|signature| signature.is_match(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pronom::{Author, ByteSequence, Endianness, PositionType};
    use indoc::indoc;
    use quick_xml::de::from_str;
    use quick_xml::se::to_string;

    #[expect(clippy::too_many_lines)]
    #[test]
    fn test_serde() -> anyhow::Result<()> {
        let xml = indoc! {r"
            <FileFormat>
              <FormatID>664</FormatID>
              <FormatName>Portable Network Graphics</FormatName>
              <FormatVersion>1.0</FormatVersion>
              <FormatAliases>PNG (1.0)</FormatAliases>
              <FormatFamilies/>
              <FormatTypes>Image (Raster)</FormatTypes>
              <FormatDisclosure/>
              <FormatDescription>Portable Network Graphics (PNG) was designed for the lossless, portable, compressed storage of raster images.  PNG provides a patent-free replacement for GIF and can also replace many common uses of TIFF. Indexed-color, grayscale, and truecolor images are supported, plus an optional alpha channel. Sample depths range from 1 to 16 bits. PNG is designed to work in online viewing applications, so it is fully streamable.  It can store gamma and chromaticity.  PNG also detects file corruption.</FormatDescription>
              <BinaryFileFormat>Binary</BinaryFileFormat>
              <ByteOrders>Big-endian (Motorola)</ByteOrders>
              <ReleaseDate/>
              <WithdrawnDate/>
              <ProvenanceSourceId>0</ProvenanceSourceId>
              <ProvenanceName>The National Archives and Records Administration / The National Archives and Records Administration</ProvenanceName>
              <ProvenanceSourceDate>11 Mar 2005</ProvenanceSourceDate>
              <ProvenanceDescription>Specifications link: http://tools.ietf.org/pdf/rfc2083.pdf</ProvenanceDescription>
              <LastUpdatedDate>11 Jun 2012</LastUpdatedDate>
              <FormatNote/>
              <FormatRisk/>
              <TechnicalEnvironment/>
              <FileFormatIdentifier>
                <Identifier>fmt/11</Identifier>
                <IdentifierType>PUID</IdentifierType>
              </FileFormatIdentifier>
              <FileFormatIdentifier>
                <Identifier>image/png</Identifier>
                <IdentifierType>MIME</IdentifierType>
              </FileFormatIdentifier>
              <FileFormatIdentifier>
                <Identifier>public.png</Identifier>
                <IdentifierType>Apple Uniform Type Identifier</IdentifierType>
              </FileFormatIdentifier>
              <ExternalSignature>
                <ExternalSignatureID>761</ExternalSignatureID>
                <Signature>png</Signature>
                <SignatureType>File extension</SignatureType>
              </ExternalSignature>
              <InternalSignature>
                <SignatureID>58</SignatureID>
                <SignatureName>PNG 1.0</SignatureName>
                <SignatureNote>Signature + IHDR chunk at BOF, IEND chunk at EOF</SignatureNote>
                <ByteSequence>
                  <ByteSequenceID>161</ByteSequenceID>
                  <PositionType>Absolute from BOF</PositionType>
                  <Offset>0</Offset>
                  <MaxOffset>0</MaxOffset>
                  <IndirectOffsetLocation/>
                  <IndirectOffsetLength/>
                  <Endianness/>
                  <ByteSequenceValue>89504E470D0A1A0A0000000D49484452</ByteSequenceValue>
                </ByteSequence>
                <ByteSequence>
                  <ByteSequenceID>162</ByteSequenceID>
                  <PositionType>Absolute from EOF</PositionType>
                  <Offset>0</Offset>
                  <MaxOffset>4</MaxOffset>
                  <IndirectOffsetLocation/>
                  <IndirectOffsetLength/>
                  <Endianness/>
                  <ByteSequenceValue>0000000049454E44AE426082</ByteSequenceValue>
                </ByteSequence>
              </InternalSignature>
              <RelatedFormat>
                <RelationshipType>Has lower priority than</RelationshipType>
                <RelatedFormatID>665</RelatedFormatID>
                <RelatedFormatName>Portable Network Graphics</RelatedFormatName>
                <RelatedFormatVersion>1.1</RelatedFormatVersion>
              </RelatedFormat>
              <RelatedFormat>
                <RelationshipType>Has lower priority than</RelationshipType>
                <RelatedFormatID>666</RelatedFormatID>
                <RelatedFormatName>Portable Network Graphics</RelatedFormatName>
                <RelatedFormatVersion>1.2</RelatedFormatVersion>
              </RelatedFormat>
              <RelatedFormat>
                <RelationshipType>Has lower priority than</RelationshipType>
                <RelatedFormatID>1740</RelatedFormatID>
                <RelatedFormatName>Animated Portable Network Graphics</RelatedFormatName>
                <RelatedFormatVersion/>
              </RelatedFormat>
              <RelatedFormat>
                <RelationshipType>Is previous version of</RelationshipType>
                <RelatedFormatID>665</RelatedFormatID>
                <RelatedFormatName>Portable Network Graphics</RelatedFormatName>
                <RelatedFormatVersion>1.1</RelatedFormatVersion>
              </RelatedFormat>
            </FileFormat>
        "};
        let file_format: FileFormat = from_str(xml)?;

        // Test serialization
        let xml = to_string(&file_format)?;
        let file_format: FileFormat = from_str(xml.as_str())?;
        assert_eq!(file_format.id(), 664);
        assert_eq!(file_format.puid(), "fmt/11");
        assert_eq!(file_format.name(), "Portable Network Graphics");
        assert_eq!(file_format.version(), "1.0");
        assert_eq!(file_format.aliases(), "PNG (1.0)");
        assert_eq!(file_format.families(), "");
        assert_eq!(file_format.types(), "Image (Raster)");
        assert_eq!(file_format.disclosure(), "");
        assert_eq!(
            file_format.description(),
            "Portable Network Graphics (PNG) was designed for the lossless, portable, compressed storage of raster images.  PNG provides a patent-free replacement for GIF and can also replace many common uses of TIFF. Indexed-color, grayscale, and truecolor images are supported, plus an optional alpha channel. Sample depths range from 1 to 16 bits. PNG is designed to work in online viewing applications, so it is fully streamable.  It can store gamma and chromaticity.  PNG also detects file corruption."
        );
        assert_eq!(file_format.binary_file_format(), "Binary");
        assert_eq!(file_format.byte_orders(), "Big-endian (Motorola)");
        assert_eq!(file_format.release_date(), &None);
        assert_eq!(file_format.withdrawn_date(), &None);
        assert_eq!(file_format.provenance_source_id(), 0);
        assert_eq!(
            file_format.provenance_name(),
            "The National Archives and Records Administration / The National Archives and Records Administration"
        );
        assert_eq!(
            file_format.provenance_source_date(),
            &Date::new(2005, 3, 11).expect("Invalid date")
        );
        assert_eq!(
            file_format.provenance_description(),
            "Specifications link: http://tools.ietf.org/pdf/rfc2083.pdf"
        );
        assert_eq!(
            file_format.last_updated_date(),
            &Date::new(2012, 6, 11).expect("Invalid date")
        );
        assert_eq!(file_format.note(), "");
        assert_eq!(file_format.risk(), "");
        assert_eq!(file_format.technical_environment(), "");
        assert_eq!(file_format.file_format_identifiers().len(), 3);
        assert_eq!(file_format.documents().len(), 0);
        assert_eq!(file_format.external_signatures().len(), 1);
        assert_eq!(file_format.internal_signatures().len(), 1);
        assert_eq!(file_format.related_formats().len(), 4);
        assert_eq!(file_format.compression_types().len(), 0);
        Ok(())
    }

    #[test]
    fn test_new() {
        let file_format = FileFormat::new(
            664,
            "Portable Network Graphics",
            "1.0",
            "PNG (1.0)",
            "",
            "Image (Raster)",
            "",
            "Portable Network Graphics (PNG) was designed for the lossless, portable, compressed storage of raster images.  PNG provides a patent-free replacement for GIF and can also replace many common uses of TIFF. Indexed-color, grayscale, and truecolor images are supported, plus an optional alpha channel. Sample depths range from 1 to 16 bits. PNG is designed to work in online viewing applications, so it is fully streamable.  It can store gamma and chromaticity.  PNG also detects file corruption.",
            "Binary",
            "Big-endian (Motorola)",
            None,
            None,
            0,
            "The National Archives and Records Administration / The National Archives and Records Administration",
            Date::new(2005, 3, 11).expect("Invalid date"),
            "Specifications link: http://tools.ietf.org/pdf/rfc2083.pdf",
            Date::new(2012, 6, 11).expect("Invalid date"),
            "",
            "",
            "",
            vec![
                DocumentIdentifier::new("fmt/11", "PUID"),
                DocumentIdentifier::new("image/png", "MIME"),
                DocumentIdentifier::new("public.png", "Apple Uniform Type Identifier"),
            ],
            vec![],
            vec![
                ExternalSignature::new(761, "png", "File extension"),
            ],
            vec![],
            vec![],
            vec![],
        );

        assert_eq!(file_format.id(), 664);
        assert_eq!(file_format.puid(), "fmt/11");
        assert_eq!(file_format.name(), "Portable Network Graphics");
        assert_eq!(file_format.version(), "1.0");
        assert_eq!(file_format.aliases(), "PNG (1.0)");
        assert_eq!(file_format.families(), "");
        assert_eq!(file_format.types(), "Image (Raster)");
        assert_eq!(file_format.disclosure(), "");
        assert_eq!(
            file_format.description(),
            "Portable Network Graphics (PNG) was designed for the lossless, portable, compressed storage of raster images.  PNG provides a patent-free replacement for GIF and can also replace many common uses of TIFF. Indexed-color, grayscale, and truecolor images are supported, plus an optional alpha channel. Sample depths range from 1 to 16 bits. PNG is designed to work in online viewing applications, so it is fully streamable.  It can store gamma and chromaticity.  PNG also detects file corruption."
        );
        assert_eq!(file_format.binary_file_format(), "Binary");
        assert_eq!(file_format.byte_orders(), "Big-endian (Motorola)");
        assert_eq!(file_format.release_date(), &None);
        assert_eq!(file_format.withdrawn_date(), &None);
        assert_eq!(file_format.provenance_source_id(), 0);
        assert_eq!(
            file_format.provenance_name(),
            "The National Archives and Records Administration / The National Archives and Records Administration"
        );
        assert_eq!(
            file_format.provenance_source_date(),
            &Date::new(2005, 3, 11).expect("Invalid date")
        );
        assert_eq!(
            file_format.provenance_description(),
            "Specifications link: http://tools.ietf.org/pdf/rfc2083.pdf"
        );
        assert_eq!(
            file_format.last_updated_date(),
            &Date::new(2012, 6, 11).expect("Invalid date")
        );
        assert_eq!(file_format.note(), "");
        assert_eq!(file_format.risk(), "");
        assert_eq!(file_format.technical_environment(), "");
        assert_eq!(file_format.file_format_identifiers().len(), 3);
        assert_eq!(file_format.documents().len(), 0);
        assert_eq!(file_format.external_signatures().len(), 1);
        assert_eq!(file_format.internal_signatures().len(), 0);
        assert_eq!(file_format.related_formats().len(), 0);
        assert_eq!(file_format.compression_types().len(), 0);
        assert_eq!(file_format.extensions(), vec!["png"]);
        assert_eq!(file_format.media_types(), vec!["image/png"]);
    }
}
