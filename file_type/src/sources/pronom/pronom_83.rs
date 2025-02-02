use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_83: FileFormat = FileFormat {
    id: 83,
    source_type: SourceType::Pronom,
    name: "AutoCAD Drawing Template",
    extensions: &["dwt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
