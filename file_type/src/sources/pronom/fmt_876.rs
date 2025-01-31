use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_876: FileFormat = FileFormat {
    id: 1_680,
    puid: "fmt/876",
    name: "Pagemaker Document (Generic)",
    extensions: &["p65", "pmd", "pmt"],
    media_types: &["application/vnd.pagemaker"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 247,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 254,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_554,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_555,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
    ],
};
