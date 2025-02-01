use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_412: FileFormat = FileFormat {
    id: 1_160,
    puid: "fmt/412",
    name: "Microsoft Word for Windows",
    extensions: &["docx", "wbk"],
    media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.document"],
    internal_signatures: &[],
    related_formats: &[
        RelatedFormat {
            id: 2_678,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_389,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 1_391,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
    ],
};
