use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1297: FileFormat = FileFormat {
    id: 2_115,
    puid: "fmt/1297",
    name: "Calendar Creator File",
    extensions: &["cc5"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 2_116,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_114,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
