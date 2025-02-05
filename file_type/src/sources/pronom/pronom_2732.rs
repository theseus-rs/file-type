use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2732: FileFormat = FileFormat {
    id: 2_732,
    source_type: SourceType::Pronom,
    name: "GST Art File",
    extensions: &["art"],
    media_types: &[],
    signatures: &[],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 2_731,
    }],
};
