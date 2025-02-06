use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_300: FileFormat = FileFormat {
    id: 300,
    source_type: SourceType::Pronom,
    name: "Quicken Data File",
    extensions: &["abd", "qdf", "qel"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
