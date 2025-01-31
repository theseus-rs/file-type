use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_472: FileFormat = FileFormat {
    id: 1_259,
    puid: "fmt/472",
    name: "Sony Digital Voice File/Sony Memory Stick Voice File",
    extensions: &["msv", "dvf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x53, 0x5F, 0x56, 0x4F, 0x49, 0x43, 0x45, 0x00, 0x00, 0x00, 0x50, 0x01,
                    0x02, 0x00, 0x00, 0x53, 0x4F, 0x4E, 0x59, 0x20, 0x43, 0x4F, 0x52, 0x50, 0x4F,
                    0x52, 0x41, 0x54, 0x49, 0x4F, 0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_746,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_747,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_748,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
    ],
};
