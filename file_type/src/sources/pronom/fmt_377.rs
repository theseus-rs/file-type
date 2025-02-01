use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_377: FileFormat = FileFormat {
    id: 1_124,
    puid: "fmt/377",
    name: "Microsoft Visual FoxPro Report",
    extensions: &["frx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x30]),
                    Token::SingleWildcard,
                    Token::Range(&[0x01], &[0x0C]),
                    Token::Range(&[0x01], &[0x1F]),
                    Token::WildcardCount(1),
                    Token::Literal(&[0x00, 0x00, 0x00, 0x88, 0x0A, 0xE5]),
                    Token::WildcardCount(21),
                    Token::Literal(&[
                        0x50, 0x4C, 0x41, 0x54, 0x46, 0x4F, 0x52, 0x4D, 0x00, 0x00, 0x00, 0x43,
                    ]),
                    Token::WildcardCount(20),
                    Token::Literal(&[
                        0x55, 0x4E, 0x49, 0x51, 0x55, 0x45, 0x49, 0x44, 0x00, 0x00, 0x00, 0x43,
                    ]),
                    Token::WildcardCount(52),
                    Token::Literal(&[
                        0x4F, 0x42, 0x4A, 0x54, 0x59, 0x50, 0x45, 0x00, 0x00, 0x00, 0x00, 0x4E,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_121,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
