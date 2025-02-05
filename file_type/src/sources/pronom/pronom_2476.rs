use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2476: FileFormat = FileFormat {
    id: 2_476,
    source_type: SourceType::Pronom,
    name: "AGS 4 Data Format",
    extensions: &["ags"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x22]),
                    Token::Any(&[
                        &[Token::Literal(&[0x47, 0x52, 0x4F, 0x55, 0x50])],
                        &[Token::Literal(&[0x2A, 0x2A, 0x50, 0x52, 0x4F, 0x4A])],
                    ]),
                    Token::Literal(&[0x22]),
                    Token::AnyWildcard,
                    Token::AnyWildcard,
                    Token::Literal(&[0x22]),
                    Token::WildcardCountRange(0, 1),
                    Token::Literal(&[0x50, 0x52, 0x4F, 0x4A, 0x5F, 0x49, 0x44, 0x22, 0x2C, 0x22]),
                    Token::WildcardCountRange(0, 1),
                    Token::Any(&[
                        &[Token::Literal(&[
                            0x50, 0x52, 0x4F, 0x4A, 0x5F, 0x4E, 0x41, 0x4D, 0x45, 0x22, 0x2C, 0x22,
                        ])],
                        &[Token::Literal(&[
                            0x50, 0x52, 0x4F, 0x4A, 0x5F, 0x41, 0x47, 0x53, 0x22,
                        ])],
                    ]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x22]),
                    Token::WildcardCountRange(0, 1),
                    Token::Literal(&[
                        0x41, 0x42, 0x42, 0x52, 0x5F, 0x48, 0x44, 0x4E, 0x47, 0x22, 0x2C, 0x22,
                    ]),
                    Token::WildcardCountRange(0, 1),
                    Token::Literal(&[
                        0x41, 0x42, 0x42, 0x52, 0x5F, 0x43, 0x4F, 0x44, 0x45, 0x22, 0x2C, 0x22,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
