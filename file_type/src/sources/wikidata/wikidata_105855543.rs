use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855543: FileFormat = FileFormat {
    id: 105_855_543,
    puid: "wikidata/105855543",
    name: "Psion Object/OPL Output",
    extensions: &["app", "opa", "opo"],
    media_types: &[
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
                        0x4F, 0x50, 0x4C, 0x4F, 0x62, 0x6A, 0x65, 0x63, 0x74, 0x46, 0x69, 0x6C,
                        0x65, 0x2A, 0x2A, 0x00,
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
                        0x4F, 0x50, 0x4C, 0x4F, 0x62, 0x6A, 0x65, 0x63, 0x74, 0x46, 0x69, 0x6C,
                        0x65, 0x2A, 0x2A, 0x00,
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
                        0x4F, 0x50, 0x4C, 0x4F, 0x62, 0x6A, 0x65, 0x63, 0x74, 0x46, 0x69, 0x6C,
                        0x65, 0x2A, 0x2A, 0x00,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
