use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1571: FileFormat = FileFormat {
    id: 2_396,
    puid: "fmt/1571",
    name: "ISDOCX Information System Document",
    extensions: &["isdocx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 382,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_393,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_395,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
    ],
};
