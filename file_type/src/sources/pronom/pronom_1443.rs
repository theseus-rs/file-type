use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1443: FileFormat = FileFormat {
    id: 1_443,
    source_type: SourceType::Pronom,
    name: "Nullsoft Scriptable Install System",
    extensions: &["nsi"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
