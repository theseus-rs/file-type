use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_118: FileFormat = FileFormat {
    id: 118,
    source_type: SourceType::Pronom,
    name: "CorelDraw Pattern",
    extensions: &["pat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
