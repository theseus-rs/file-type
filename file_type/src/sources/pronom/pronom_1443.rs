use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1443: FileFormat = FileFormat {
    id: 1_443,
    source_type: SourceType::Pronom,
    name: "Nullsoft Scriptable Install System",
    extensions: &["nsi"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
