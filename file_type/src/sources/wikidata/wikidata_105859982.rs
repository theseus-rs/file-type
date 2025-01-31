use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859982: FileFormat = FileFormat {
    id: 105_859_982,
    puid: "wikidata/105859982",
    name: "The Software Toolworks resources archive",
    extensions: &["lst", "res", "v2l", "vgh"],
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
                        0x52, 0x49, 0x43, 0x4B, 0x42, 0x4F, 0x00, 0x76,
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
                        0x52, 0x49, 0x43, 0x4B, 0x42, 0x4F, 0x00, 0x76,
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
                        0x52, 0x49, 0x43, 0x4B, 0x42, 0x4F, 0x00, 0x76,
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
                        0x52, 0x49, 0x43, 0x4B, 0x42, 0x4F, 0x00, 0x76,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
