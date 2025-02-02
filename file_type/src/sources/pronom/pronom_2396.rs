use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2396: FileFormat = FileFormat {
    id: 2_396,
    source_type: SourceType::Pronom,
    name: "ISDOCX Information System Document",
    extensions: &["isdocx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 382,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 2_393,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 2_395,
        },
    ],
};
