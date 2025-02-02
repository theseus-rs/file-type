use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1888: FileFormat = FileFormat {
    id: 1_888,
    source_type: SourceType::Pronom,
    name: "ASP Application Directive File",
    extensions: &["asax"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x3C, 0x25, 0x40, 0x20, 0x41, 0x70, 0x70, 0x6C, 0x69, 0x63, 0x61, 0x74,
                        0x69, 0x6F, 0x6E, 0x20,
                    ]),
                    Token::Any(&[
                        &[Token::Literal(&[
                            0x43, 0x6F, 0x64, 0x65, 0x42, 0x65, 0x68, 0x69, 0x6E, 0x64,
                        ])],
                        &[Token::Literal(&[
                            0x43, 0x6F, 0x6D, 0x70, 0x69, 0x6C, 0x65, 0x72, 0x4F, 0x70, 0x74, 0x69,
                            0x6F, 0x6E, 0x73,
                        ])],
                        &[Token::Literal(&[
                            0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x69, 0x6F, 0x6E,
                        ])],
                        &[Token::Literal(&[
                            0x49, 0x6E, 0x68, 0x65, 0x72, 0x69, 0x74, 0x73,
                        ])],
                        &[Token::Literal(&[
                            0x4C, 0x61, 0x6E, 0x67, 0x75, 0x61, 0x67, 0x65,
                        ])],
                    ]),
                    Token::Literal(&[0x3D]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
