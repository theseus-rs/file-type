use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1632: FileFormat = FileFormat {
    id: 1_632,
    source_type: SourceType::Pronom,
    name: "Polygon File Format",
    extensions: &["ply"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x70, 0x6C, 0x79]),
                    Token::Any(&[
                        &[Token::Literal(&[0x0A])],
                        &[Token::Literal(&[0x0D])],
                        &[Token::Literal(&[0x0D, 0x0A])],
                    ]),
                    Token::Literal(&[0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x20]),
                    Token::Any(&[
                        &[Token::Literal(&[0x61, 0x73, 0x63, 0x69, 0x69])],
                        &[Token::Literal(&[
                            0x62, 0x69, 0x6E, 0x61, 0x72, 0x79, 0x5F, 0x6C, 0x69, 0x74, 0x74, 0x6C,
                            0x65, 0x5F, 0x65, 0x6E, 0x64, 0x69, 0x61, 0x6E,
                        ])],
                        &[Token::Literal(&[
                            0x62, 0x69, 0x6E, 0x61, 0x72, 0x79, 0x5F, 0x62, 0x69, 0x67, 0x5F, 0x65,
                            0x6E, 0x64, 0x69, 0x61, 0x6E,
                        ])],
                    ]),
                    Token::Literal(&[0x20, 0x31, 0x2E, 0x30]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
