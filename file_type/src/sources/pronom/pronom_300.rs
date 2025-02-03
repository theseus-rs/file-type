use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_300: FileFormat = FileFormat {
    id: 300,
    source_type: SourceType::Pronom,
    name: "Quicken Data File",
    extensions: &["abd", "qdf", "qel"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
