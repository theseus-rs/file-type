use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_492: FileFormat = FileFormat {
    id: 492,
    source_type: SourceType::Pronom,
    name: "Interleaf Document",
    extensions: &["doc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
