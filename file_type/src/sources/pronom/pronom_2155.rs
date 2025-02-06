use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2155: FileFormat = FileFormat {
    id: 2_155,
    source_type: SourceType::Pronom,
    name: "LEADToolsCompressed Image",
    extensions: &["cmp"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4C]),
                    Token::Any(&[&[Token::Literal(&[0x45])], &[Token::Literal(&[0x65])]]),
                    Token::Any(&[&[Token::Literal(&[0x41])], &[Token::Literal(&[0x61])]]),
                    Token::Any(&[&[Token::Literal(&[0x44])], &[Token::Literal(&[0x64])]]),
                    Token::Literal(&[0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
