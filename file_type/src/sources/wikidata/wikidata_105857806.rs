use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857806: FileFormat = FileFormat {
    id: 105_857_806,
    puid: "wikidata/105857806",
    name: "Fahrenheit game data archive",
    extensions: &["dat", "idm"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x51, 0x55, 0x41, 0x4E, 0x54, 0x49, 0x43, 0x44, 0x52, 0x45, 0x41, 0x4D,
                        0x54, 0x41, 0x42, 0x49, 0x44, 0x4D, 0x45, 0x4D,
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
                        0x51, 0x55, 0x41, 0x4E, 0x54, 0x49, 0x43, 0x44, 0x52, 0x45, 0x41, 0x4D,
                        0x54, 0x41, 0x42, 0x49, 0x44, 0x4D, 0x45, 0x4D,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
