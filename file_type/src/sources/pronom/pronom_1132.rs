use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1132: FileFormat = FileFormat {
    id: 1_132,
    source_type: SourceType::Pronom,
    name: "VICAR (Video Image Communication and Retrieval) Planetary File Format",
    extensions: &["img", "vic", "vicar"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4C, 0x42, 0x4C, 0x53, 0x49, 0x5A, 0x45, 0x3D]),
                    Token::Range(&[0x30], &[0x39]),
                    Token::WildcardCount(15),
                    Token::Literal(&[0x46, 0x4F, 0x52, 0x4D, 0x41, 0x54, 0x3D, 0x27]),
                    Token::Any(&[
                        &[Token::Literal(&[0x48, 0x41, 0x4C, 0x46])],
                        &[Token::Literal(&[0x42, 0x59, 0x54, 0x45])],
                        &[Token::Literal(&[0x46, 0x55, 0x4C, 0x4C])],
                        &[Token::Literal(&[0x52, 0x45, 0x41, 0x4C])],
                        &[Token::Literal(&[0x44, 0x4F, 0x55, 0x42])],
                        &[Token::Literal(&[0x43, 0x4F, 0x4D, 0x50])],
                        &[Token::Literal(&[0x43, 0x4F, 0x4D, 0x50, 0x4C, 0x45, 0x58])],
                        &[Token::Literal(&[0x57, 0x4F, 0x52, 0x44])],
                        &[Token::Literal(&[0x4C, 0x4F, 0x4E, 0x47])],
                    ]),
                    Token::Literal(&[0x27]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
