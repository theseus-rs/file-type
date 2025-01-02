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
    #[serde(rename = "FormatVersion")]
    version: String,
    #[serde(rename = "FormatAliases")]
    aliases: String,
    #[serde(rename = "FormatFamilies")]
    families: String,
    #[serde(rename = "FormatTypes")]
    types: String,
    #[serde(rename = "FormatDisclosure")]
    disclosure: String,
    #[serde(rename = "FormatDescription")]
    description: String,
    binary_file_format: String,
    byte_orders: String,
    #[serde(
        deserialize_with = "deserialize_option_naive_date",
        serialize_with = "serialize_option_naive_date"
    )]
    release_date: Option<Date>,
    #[serde(
        deserialize_with = "deserialize_option_naive_date",
        serialize_with = "serialize_option_naive_date"
    )]
    withdrawn_date: Option<Date>,
    provenance_source_id: usize,
    provenance_name: String,
    #[serde(
        deserialize_with = "deserialize_naive_date",
        serialize_with = "serialize_naive_date"
    )]
    provenance_source_date: Date,
    provenance_description: String,
    #[serde(
        deserialize_with = "deserialize_naive_date",
        serialize_with = "serialize_naive_date"
    )]
    last_updated_date: Date,
    #[serde(rename = "FormatNote")]
    note: String,
    #[serde(rename = "FormatRisk")]
    risk: String,
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
    pub fn extensions(&self) -> Vec<String> {
        self.external_signatures
            .iter()
            .map(|signature| signature.signature().to_string())
            .collect()
    }

    /// Get media types
    #[must_use]
    pub fn media_types(&self) -> Vec<String> {
        self.file_format_identifiers
            .iter()
            .filter_map(|identifier| {
                if identifier.r#type() == "MIME" {
                    Some(identifier.identifier().to_string())
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
