use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105857723: FileFormat = FileFormat {
    id: 105_857_723,
    puid: "wikidata/105857723",
    name: "Infinity Engine World Map (v1.0)",
    extensions: &["wmap", "wmp"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x4D, 0x41, 0x50, 0x56, 0x31, 0x2E, 0x30,
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
                        0x57, 0x4D, 0x41, 0x50, 0x56, 0x31, 0x2E, 0x30,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
