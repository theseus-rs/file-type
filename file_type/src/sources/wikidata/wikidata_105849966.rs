use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849966: FileFormat = FileFormat {
    id: 105_849_966,
    source_type: SourceType::Wikidata,
    name: "Clam AntiVirus Database",
    extensions: &["cld", "cud", "cvd", "info"],
    media_types: &["application/octet-stream"],
    signatures: &[
        Signature {
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
        Signature {
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
        Signature {
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
        Signature {
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
