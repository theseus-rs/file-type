use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1160: FileFormat = FileFormat {
    id: 1_160,
    source_type: SourceType::Pronom,
    name: "Microsoft Word for Windows",
    extensions: &["docx", "wbk"],
    media_types: &["application/vnd.openxmlformats-officedocument.wordprocessingml.document"],
    signatures: &[],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_678,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 1_389,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 1_391,
        },
    ],
};
