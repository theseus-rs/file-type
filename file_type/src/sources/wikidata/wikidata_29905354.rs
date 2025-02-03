use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29905354: FileFormat = FileFormat {
    id: 29_905_354,
    source_type: SourceType::Wikidata,
    name: "Self-contained Information Retention Format",
    extensions: &["json", "xml"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
