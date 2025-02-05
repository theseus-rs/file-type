use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1373: FileFormat = FileFormat {
    id: 1_373,
    source_type: SourceType::Pronom,
    name: "MPEG-2 Transport Stream",
    extensions: &["m2t", "ts", "m2ts"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x47]),
                    Token::WildcardCount(187),
                    Token::Literal(&[0x47]),
                    Token::WildcardCount(187),
                    Token::Literal(&[0x47]),
                    Token::WildcardCount(187),
                    Token::Literal(&[0x47]),
                    Token::WildcardCount(187),
                    Token::Literal(&[0x47]),
                    Token::WildcardCount(187),
                    Token::Literal(&[0x47]),
                    Token::WildcardCount(187),
                    Token::Literal(&[0x47]),
                    Token::WildcardCount(187),
                    Token::Literal(&[0x47]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
