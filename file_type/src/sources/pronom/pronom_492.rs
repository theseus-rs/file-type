use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_492: FileFormat = FileFormat {
    id: 492,
    source_type: SourceType::Pronom,
    name: "Interleaf Document",
    extensions: &["doc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
