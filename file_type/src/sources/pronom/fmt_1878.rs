use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1878: FileFormat = FileFormat {
    id: 2_732,
    puid: "fmt/1878",
    name: "GST Art File",
    extensions: &["art"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[RelatedFormat {
        id: 2_731,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
