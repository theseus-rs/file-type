use crate::format::compression_type::CompressionType;
use crate::format::document_identifier::DocumentIdentifier;
use crate::format::external_signature::ExternalSignature;
use crate::format::internal_signature::InternalSignature;
use crate::format::related_format::RelatedFormat;
use serde::{Deserialize, Serialize};

/// The types of file format
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub(crate) enum FormatTypes {
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

/// A file format and its associated information
#[expect(clippy::struct_field_names)]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default, rename_all = "PascalCase")]
pub(crate) struct FileFormat {
    #[serde(rename = "FormatID")]
    pub(crate) id: usize,
    #[serde(rename = "FormatName")]
    pub(crate) name: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "FormatVersion")]
    pub(crate) version: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "FormatAliases")]
    pub(crate) aliases: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "FormatFamilies")]
    pub(crate) families: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "FormatTypes")]
    pub(crate) types: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "FormatDisclosure")]
    pub(crate) disclosure: String,
    #[serde(skip_serializing_if = "String::is_empty", rename = "FormatDescription")]
    pub(crate) description: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) binary_file_format: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub(crate) byte_orders: String,
    #[serde(rename = "FileFormatIdentifier")]
    pub(crate) file_format_identifiers: Vec<DocumentIdentifier>,
    #[serde(rename = "ExternalSignature")]
    pub(crate) external_signatures: Vec<ExternalSignature>,
    #[serde(rename = "InternalSignature")]
    pub(crate) internal_signatures: Vec<InternalSignature>,
    #[serde(rename = "RelatedFormat")]
    pub(crate) related_formats: Vec<RelatedFormat>,
    #[serde(rename = "CompressionType")]
    pub(crate) compression_types: Vec<CompressionType>,
}

impl FileFormat {
    /// Convert to the type used at runtime
    pub fn to_runtime_type(&self) -> file_type::Result<file_type::format::FileFormat> {
        let file_format_identifier = self
            .file_format_identifiers
            .iter()
            .find(|identifier| identifier.r#type == "PUID");
        let puid = if let Some(file_format_identifier) = file_format_identifier {
            file_format_identifier.identifier.clone()
        } else {
            String::new()
        };

        let extensions = self
            .external_signatures
            .iter()
            .map(|external_signature| external_signature.signature.clone())
            .map(|extension| {
                let extension: &'static str = Box::leak(extension.into_boxed_str());
                extension
            })
            .collect::<Vec<&str>>();

        let media_types = self
            .file_format_identifiers
            .iter()
            .filter_map(|identifier| {
                if identifier.r#type == "MIME" {
                    Some(identifier.identifier.clone())
                } else {
                    None
                }
            })
            .map(|media_type| {
                let media_type: &'static str = Box::leak(media_type.into_boxed_str());
                media_type
            })
            .collect::<Vec<&str>>();

        let mut internal_signatures = Vec::new();
        for internal_signature in &self.internal_signatures {
            internal_signatures.push(internal_signature.to_runtime_type()?);
        }

        let mut related_formats = Vec::new();
        for related_format in &self.related_formats {
            related_formats.push(related_format.to_runtime_type()?);
        }

        let file_format = file_type::format::FileFormat {
            id: self.id,
            puid: Box::leak(puid.into_boxed_str()),
            name: Box::leak(self.name.clone().into_boxed_str()),
            extensions: Box::leak(extensions.into_boxed_slice()),
            media_types: Box::leak(media_types.into_boxed_slice()),
            internal_signatures: Box::leak(internal_signatures.into_boxed_slice()),
            related_formats: Box::leak(related_formats.into_boxed_slice()),
        };

        Ok(file_format)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use quick_xml::de::from_str;
    use quick_xml::se::to_string;

    #[test]
    fn test_serde() -> anyhow::Result<()> {
        let xml = indoc! {r"
            <FileFormat>
              <FormatID>664</FormatID>
              <FormatName>Portable Network Graphics</FormatName>
              <FormatVersion>1.0</FormatVersion>
              <FormatAliases>PNG (1.0)</FormatAliases>
              <FormatTypes>Image (Raster)</FormatTypes>
              <FormatDescription>Portable Network Graphics (PNG) was designed for the lossless, portable, compressed storage of raster images.  PNG provides a patent-free replacement for GIF and can also replace many common uses of TIFF. Indexed-color, grayscale, and truecolor images are supported, plus an optional alpha channel. Sample depths range from 1 to 16 bits. PNG is designed to work in online viewing applications, so it is fully streamable.  It can store gamma and chromaticity.  PNG also detects file corruption.</FormatDescription>
              <BinaryFileFormat>Binary</BinaryFileFormat>
              <ByteOrders>Big-endian (Motorola)</ByteOrders>
              <ProvenanceSourceId>0</ProvenanceSourceId>
              <ProvenanceName>The National Archives and Records Administration / The National Archives and Records Administration</ProvenanceName>
              <ProvenanceSourceDate>11 Mar 2005</ProvenanceSourceDate>
              <ProvenanceDescription>Specifications link: http://tools.ietf.org/pdf/rfc2083.pdf</ProvenanceDescription>
              <LastUpdatedDate>11 Jun 2012</LastUpdatedDate>
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
                  <ByteSequenceValue>89504E470D0A1A0A0000000D49484452</ByteSequenceValue>
                </ByteSequence>
                <ByteSequence>
                  <ByteSequenceID>162</ByteSequenceID>
                  <PositionType>Absolute from EOF</PositionType>
                  <Offset>0</Offset>
                  <MaxOffset>4</MaxOffset>
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
        assert_eq!(file_format.id, 664);
        assert_eq!(file_format.name, "Portable Network Graphics");
        assert_eq!(file_format.version, "1.0");
        assert_eq!(file_format.aliases, "PNG (1.0)");
        assert_eq!(file_format.families, "");
        assert_eq!(file_format.types, "Image (Raster)");
        assert_eq!(file_format.disclosure, "");
        assert_eq!(
            file_format.description,
            "Portable Network Graphics (PNG) was designed for the lossless, portable, compressed storage of raster images.  PNG provides a patent-free replacement for GIF and can also replace many common uses of TIFF. Indexed-color, grayscale, and truecolor images are supported, plus an optional alpha channel. Sample depths range from 1 to 16 bits. PNG is designed to work in online viewing applications, so it is fully streamable.  It can store gamma and chromaticity.  PNG also detects file corruption."
        );
        assert_eq!(file_format.binary_file_format, "Binary");
        assert_eq!(file_format.byte_orders, "Big-endian (Motorola)");
        assert_eq!(file_format.file_format_identifiers.len(), 3);
        assert_eq!(file_format.external_signatures.len(), 1);
        assert_eq!(file_format.internal_signatures.len(), 1);
        assert_eq!(file_format.related_formats.len(), 4);
        assert_eq!(file_format.compression_types.len(), 0);
        Ok(())
    }
}
