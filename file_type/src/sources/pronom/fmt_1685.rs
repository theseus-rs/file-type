use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1685: FileFormat = FileFormat {
    id: 2_521,
    puid: "fmt/1685",
    name: "EndNote Compressed Library",
    extensions: &["enlx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 382,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_518,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 2_519,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
