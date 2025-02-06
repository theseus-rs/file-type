use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_83: FileFormat = FileFormat {
    id: 83,
    source_type: SourceType::Pronom,
    name: "AutoCAD Drawing Template",
    extensions: &["dwt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
