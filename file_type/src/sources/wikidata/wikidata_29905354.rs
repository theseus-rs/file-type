use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_29905354: FileFormat = FileFormat {
    id: 29_905_354,
    source_type: SourceType::Wikidata,
    name: "Self-contained Information Retention Format",
    extensions: &["json", "xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
