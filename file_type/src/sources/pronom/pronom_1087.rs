use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1087: FileFormat = FileFormat {
    id: 1_087,
    source_type: SourceType::Pronom,
    name: "Microsoft Project Export File",
    extensions: &["mpx"],
    media_types: &["application/x-project"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4D, 0x50, 0x58]),
                    Token::Any(&[&[Token::Literal(&[0x2C])], &[Token::Literal(&[0x3B])]]),
                    Token::WildcardCountRange(1, 50),
                    Token::Any(&[&[Token::Literal(&[0x2C])], &[Token::Literal(&[0x3B])]]),
                    Token::Literal(&[0x34]),
                    Token::Any(&[&[Token::Literal(&[0x2C])], &[Token::Literal(&[0x2E])]]),
                    Token::Literal(&[0x30]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
