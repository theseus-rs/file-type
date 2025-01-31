use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1564: FileFormat = FileFormat {
    id: 2_389,
    puid: "fmt/1564",
    name: "Associated Signature Container Extended (ASiC-E)",
    extensions: &["asice", "sce"],
    media_types: &["application/vnd.etsi.asic-e+zip"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 2_069,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_160,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_069,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 2_160,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
    ],
};
