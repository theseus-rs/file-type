use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_383: FileFormat = FileFormat {
    id: 657,
    puid: "x-fmt/383",
    name: "Flexible Image Transport System",
    extensions: &["fits"],
    media_types: &["application/fits", "image/fits"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x53, 0x49, 0x4D, 0x50, 0x4C, 0x45, 0x20, 0x20, 0x3D, 0x20, 0x20, 0x20,
                        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
                        0x20, 0x20, 0x20, 0x20, 0x20, 0x54,
                    ]),
                    Token::WildcardCount(50),
                    Token::Literal(&[0x42, 0x49, 0x54, 0x50, 0x49, 0x58, 0x20, 0x20, 0x3D]),
                    Token::WildcardCount(19),
                    Token::Any(&[
                        &[Token::Literal(&[0x20, 0x38])],
                        &[Token::Literal(&[0x2B, 0x38])],
                        &[Token::Literal(&[0x30, 0x38])],
                        &[Token::Literal(&[0x31, 0x36])],
                        &[Token::Literal(&[0x33, 0x32])],
                        &[Token::Literal(&[0x36, 0x34])],
                    ]),
                    Token::WildcardCount(50),
                    Token::Literal(&[0x4E, 0x41, 0x58, 0x49, 0x53, 0x20, 0x20, 0x20, 0x3D]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
