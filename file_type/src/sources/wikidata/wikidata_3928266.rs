use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3928266: FileFormat = FileFormat {
    id: 3_928_266,
    puid: "wikidata/3928266",
    name: "RF64",
    extensions: &["rf64", "rf64", "wav", "wav"],
    media_types: &[
        "audio/vnd.wave",
        "audio/vnd.wave",
        "audio/vnd.wave",
        "audio/vnd.wave",
    ],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x46, 0x36, 0x34, 0xFF, 0xFF, 0xFF, 0xFF, 0x57, 0x41, 0x56, 0x45,
                        0x64, 0x73, 0x36, 0x34,
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
                        0x52, 0x46, 0x36, 0x34, 0xFF, 0xFF, 0xFF, 0xFF, 0x57, 0x41, 0x56, 0x45,
                        0x64, 0x73, 0x36, 0x34,
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
                        0x52, 0x46, 0x36, 0x34, 0xFF, 0xFF, 0xFF, 0xFF, 0x57, 0x41, 0x56, 0x45,
                        0x64, 0x73, 0x36, 0x34,
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
                        0x52, 0x46, 0x36, 0x34, 0xFF, 0xFF, 0xFF, 0xFF, 0x57, 0x41, 0x56, 0x45,
                        0x64, 0x73, 0x36, 0x34,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
