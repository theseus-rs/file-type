use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_640: FileFormat = FileFormat {
    id: 1_439,
    puid: "fmt/640",
    name: "MPEG-2 Elementary Stream",
    extensions: &["mpg", "mpeg", "m2v"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x00, 0x01, 0xB3]),
                    Token::WildcardCountRange(8, 256),
                    Token::Literal(&[0x00, 0x00, 0x01, 0xB5]),
                    Token::WildcardCountRange(6, 256),
                    Token::Literal(&[0x00, 0x00, 0x01, 0xB8]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 660,
            relationship_type: RelationshipType::CanBeContainedBy,
        },
        RelatedFormat {
            id: 1_448,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
