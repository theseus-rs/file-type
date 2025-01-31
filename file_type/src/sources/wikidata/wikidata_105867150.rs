use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867150: FileFormat = FileFormat {
    id: 105_867_150,
    puid: "wikidata/105867150",
    name: "NeoGeo game cartridge (var 1)",
    extensions: &["ngc", "ngp", "npc"],
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
                        0x20, 0x4C, 0x49, 0x43, 0x45, 0x4E, 0x53, 0x45, 0x44, 0x20, 0x42, 0x59,
                        0x20, 0x53, 0x4E, 0x4B, 0x20, 0x43, 0x4F, 0x52, 0x50, 0x4F, 0x52, 0x41,
                        0x54, 0x49, 0x4F, 0x4E,
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
                        0x20, 0x4C, 0x49, 0x43, 0x45, 0x4E, 0x53, 0x45, 0x44, 0x20, 0x42, 0x59,
                        0x20, 0x53, 0x4E, 0x4B, 0x20, 0x43, 0x4F, 0x52, 0x50, 0x4F, 0x52, 0x41,
                        0x54, 0x49, 0x4F, 0x4E,
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
                        0x20, 0x4C, 0x49, 0x43, 0x45, 0x4E, 0x53, 0x45, 0x44, 0x20, 0x42, 0x59,
                        0x20, 0x53, 0x4E, 0x4B, 0x20, 0x43, 0x4F, 0x52, 0x50, 0x4F, 0x52, 0x41,
                        0x54, 0x49, 0x4F, 0x4E,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
