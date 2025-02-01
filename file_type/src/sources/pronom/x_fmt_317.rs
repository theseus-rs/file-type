use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_317: FileFormat = FileFormat {
    id: 476,
    puid: "x-fmt/317",
    name: "ESRI Arc/View Project",
    extensions: &["apr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x2F, 0x32, 0x2E]),
                    Token::Range(&[0x30], &[0x33]),
                    Token::Any(&[&[Token::Literal(&[0x0D, 0x0A])], &[Token::Literal(&[0x0A])]]),
                    Token::Literal(&[0x28, 0x50, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x2E, 0x31]),
                    Token::Any(&[&[Token::Literal(&[0x0D, 0x0A])], &[Token::Literal(&[0x0A])]]),
                    Token::Literal(&[0x09, 0x4E, 0x61, 0x6D, 0x65, 0x3A, 0x09, 0x22]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
