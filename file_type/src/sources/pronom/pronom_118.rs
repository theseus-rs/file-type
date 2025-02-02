use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_118: FileFormat = FileFormat {
    id: 118,
    source_type: SourceType::Pronom,
    name: "CorelDraw Pattern",
    extensions: &["pat"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
