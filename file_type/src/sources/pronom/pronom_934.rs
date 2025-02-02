use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_934: FileFormat = FileFormat {
    id: 934,
    source_type: SourceType::Pronom,
    name: "Binary File",
    extensions: &["bin"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 2_439,
    }],
};
