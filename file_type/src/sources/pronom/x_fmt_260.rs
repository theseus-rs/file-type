use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_260: FileFormat = FileFormat {
    id: 378,
    puid: "x-fmt/260",
    name: "WordStar for MS-DOS Document",
    extensions: &["ws", "ws4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 285,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 542,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
