use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_698: FileFormat = FileFormat {
    id: 1_497,
    puid: "fmt/698",
    name: "Standard for the Exchange of Product model data",
    extensions: &["step", "stp", "p21"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x53, 0x4F, 0x2D, 0x31, 0x30, 0x33, 0x30, 0x33, 0x2D, 0x32, 0x31, 0x3B,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_458,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_498,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_499,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
    ],
};
