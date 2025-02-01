use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105867287: FileFormat = FileFormat {
    id: 105_867_287,
    puid: "wikidata/105867287",
    name: "NeoGeo game cartridge (var 2)",
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
                        0x43, 0x4F, 0x50, 0x59, 0x52, 0x49, 0x47, 0x48, 0x54, 0x20, 0x42, 0x59,
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
                        0x43, 0x4F, 0x50, 0x59, 0x52, 0x49, 0x47, 0x48, 0x54, 0x20, 0x42, 0x59,
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
                        0x43, 0x4F, 0x50, 0x59, 0x52, 0x49, 0x47, 0x48, 0x54, 0x20, 0x42, 0x59,
                        0x20, 0x53, 0x4E, 0x4B, 0x20, 0x43, 0x4F, 0x52, 0x50, 0x4F, 0x52, 0x41,
                        0x54, 0x49, 0x4F, 0x4E,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
