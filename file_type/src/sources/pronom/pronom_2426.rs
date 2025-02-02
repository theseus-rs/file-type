use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2426: FileFormat = FileFormat {
    id: 2_426,
    source_type: SourceType::Pronom,
    name: "R Program File",
    extensions: &["r"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
