use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_378: FileFormat = FileFormat {
    id: 378,
    source_type: SourceType::Pronom,
    name: "WordStar for MS-DOS Document",
    extensions: &["ws", "ws4"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 285,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 542,
        },
    ],
};
