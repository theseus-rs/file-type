use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2389: FileFormat = FileFormat {
    id: 2_389,
    source_type: SourceType::Pronom,
    name: "Associated Signature Container Extended (ASiC-E)",
    extensions: &["asice", "sce"],
    media_types: &["application/vnd.etsi.asic-e+zip"],
    signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_069,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_160,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 2_069,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 2_160,
        },
    ],
};
