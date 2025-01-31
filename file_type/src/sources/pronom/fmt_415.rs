use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_415: FileFormat = FileFormat {
    id: 1_194,
    puid: "fmt/415",
    name: "Cinema 4D",
    extensions: &["c4d"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x46, 0x4F, 0x52, 0x4D]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x4D, 0x43, 0x34, 0x44]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 221,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_327,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
    ],
};
