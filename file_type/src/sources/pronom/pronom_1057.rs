use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1057: FileFormat = FileFormat {
    id: 1_057,
    source_type: SourceType::Pronom,
    name: "Open Financial Exchange",
    extensions: &["ofx", "qfx"],
    media_types: &["application/x-ofx"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4F, 0x46, 0x58, 0x48, 0x45, 0x41, 0x44, 0x45, 0x52, 0x3D]),
                    Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                    Token::Literal(&[0x32, 0x30, 0x30]),
                    Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                    Token::Literal(&[0x0D, 0x0A, 0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x3D]),
                    Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                    Token::Literal(&[0x32, 0x30, 0x33]),
                    Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                    Token::Literal(&[
                        0x0D, 0x0A, 0x53, 0x45, 0x43, 0x55, 0x52, 0x49, 0x54, 0x59, 0x3D,
                    ]),
                    Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                    Token::AnyWildcard,
                    Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                    Token::Literal(&[
                        0x0D, 0x0A, 0x4F, 0x4C, 0x44, 0x46, 0x49, 0x4C, 0x45, 0x55, 0x49, 0x44,
                        0x3D,
                    ]),
                    Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                    Token::AnyWildcard,
                    Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                    Token::Literal(&[
                        0x0D, 0x0A, 0x4E, 0x45, 0x57, 0x46, 0x49, 0x4C, 0x45, 0x55, 0x49, 0x44,
                        0x3D,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
