use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858505: FileFormat = FileFormat {
    id: 105_858_505,
    puid: "wikidata/105858505",
    name: "MSX BASIC Graphics bitmap (screen 2)",
    extensions: &["grp", "sc2"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0x00, 0x00, 0xFF, 0x37, 0x00, 0x00])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFE, 0x00, 0x00, 0xFF, 0x37, 0x00, 0x00])],
                },
            }],
        },
    ],
    related_formats: &[],
};
