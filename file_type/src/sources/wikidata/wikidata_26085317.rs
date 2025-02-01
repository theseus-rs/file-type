use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_26085317: FileFormat = FileFormat {
    id: 26_085_317,
    puid: "wikidata/26085317",
    name: "Portable Document Format, version 1.7",
    extensions: &["pdf", "pdf"],
    media_types: &["application/pdf", "application/pdf"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x25, 0x50, 0x44, 0x46, 0x2D, 0x31, 0x2E, 0x37,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x25, 0x25, 0x45, 0x4F]),
                        Token::Any(&[
                            &[Token::Literal(&[0x46])],
                            &[Token::Literal(&[0x46, 0x0A])],
                            &[Token::Literal(&[0x46, 0x0D])],
                            &[Token::Literal(&[0x46, 0x0D, 0x0A])],
                            &[Token::Literal(&[0x46, 0x0D, 0x00])],
                        ]),
                    ],
                },
            }],
        },
    ],
    related_formats: &[],
};
