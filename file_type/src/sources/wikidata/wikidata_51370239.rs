use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51370239: FileFormat = FileFormat {
    id: 51_370_239,
    puid: "wikidata/51370239",
    name: "Pocket Word Document",
    extensions: &["psw", "psw", "pwd", "pwd"],
    media_types: &[
        "application/x-pocket-word",
        "application/x-pocket-word",
        "application/x-pocket-word",
        "application/x-pocket-word",
    ],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7B, 0x5C, 0x70, 0x77, 0x69])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7B, 0x5C, 0x70, 0x77, 0x64, 0x32])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7B, 0x5C, 0x70, 0x77, 0x69])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x7B, 0x5C, 0x70, 0x77, 0x64, 0x32])],
                },
            }],
        },
    ],
    related_formats: &[],
};
