use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_385: FileFormat = FileFormat {
    id: 385,
    source_type: SourceType::Pronom,
    name: "Tape Archive Format",
    extensions: &["tar"],
    media_types: &["application/x-tar"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Range(&[0x21], &[0xEF]),
                    Token::WildcardCount(104),
                    Token::Range(&[0x30], &[0x37]),
                    Token::Range(&[0x20], &[0x37]),
                    Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x20])]]),
                    Token::WildcardCount(5),
                    Token::Range(&[0x30], &[0x37]),
                    Token::Range(&[0x20], &[0x37]),
                    Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x20])]]),
                    Token::WildcardCount(5),
                    Token::Range(&[0x30], &[0x37]),
                    Token::Range(&[0x20], &[0x37]),
                    Token::Literal(&[0x00]),
                    Token::WildcardCount(10),
                    Token::Range(&[0x30], &[0x37]),
                    Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x20])]]),
                    Token::WildcardCount(10),
                    Token::Range(&[0x30], &[0x37]),
                    Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x20])]]),
                    Token::WildcardCount(5),
                    Token::Range(&[0x30], &[0x37]),
                    Token::Range(&[0x00], &[0x37]),
                    Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x20])]]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasLowerPriorityThan,
        id: 795,
    }],
};
