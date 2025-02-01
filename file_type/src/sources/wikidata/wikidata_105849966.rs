use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849966: FileFormat = FileFormat {
    id: 105_849_966,
    puid: "wikidata/105849966",
    name: "Clam AntiVirus Database",
    extensions: &["cld", "cud", "cvd", "info"],
    media_types: &[
        "application/octet-stream",
        "application/octet-stream",
        "application/octet-stream",
        "application/octet-stream",
    ],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x6C, 0x61, 0x6D, 0x41, 0x56, 0x2D, 0x56, 0x44, 0x42, 0x3A,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x6C, 0x61, 0x6D, 0x41, 0x56, 0x2D, 0x56, 0x44, 0x42, 0x3A,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x6C, 0x61, 0x6D, 0x41, 0x56, 0x2D, 0x56, 0x44, 0x42, 0x3A,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x43, 0x6C, 0x61, 0x6D, 0x41, 0x56, 0x2D, 0x56, 0x44, 0x42, 0x3A,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
