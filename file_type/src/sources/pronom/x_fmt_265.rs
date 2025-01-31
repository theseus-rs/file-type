use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_265: FileFormat = FileFormat {
    id: 385,
    puid: "x-fmt/265",
    name: "Tape Archive Format",
    extensions: &["tar"],
    media_types: &["application/x-tar"],
    internal_signatures: &[InternalSignature {
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
        id: 795,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};
