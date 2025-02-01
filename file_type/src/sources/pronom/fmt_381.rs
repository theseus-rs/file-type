use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_381: FileFormat = FileFormat {
    id: 1_128,
    puid: "fmt/381",
    name: "FoxPro Project",
    extensions: &["pjx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xF5]),
                    Token::SingleWildcard,
                    Token::Range(&[0x01], &[0x0C]),
                    Token::Range(&[0x01], &[0x1F]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x01, 0x04, 0x88]),
                    Token::WildcardCount(21),
                    Token::Literal(&[
                        0x4E, 0x41, 0x4D, 0x45, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4D,
                    ]),
                    Token::WildcardCount(20),
                    Token::Literal(&[
                        0x54, 0x59, 0x50, 0x45, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x43,
                    ]),
                    Token::WildcardCount(20),
                    Token::Literal(&[
                        0x54, 0x49, 0x4D, 0x45, 0x53, 0x54, 0x41, 0x4D, 0x50, 0x00, 0x00, 0x4E,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_120,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
