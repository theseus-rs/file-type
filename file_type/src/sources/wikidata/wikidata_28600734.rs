use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28600734: FileFormat = FileFormat {
    id: 28_600_734,
    puid: "wikidata/28600734",
    name: "ESRI Arc/Info Export File",
    extensions: &["X00", "e00", "x00"],
    media_types: &["text/plain", "text/plain", "text/plain"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x58, 0x50, 0x20, 0x20, 0x30, 0x20])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x58, 0x50, 0x20, 0x20, 0x30, 0x20])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x58, 0x50, 0x20, 0x20, 0x30, 0x20])],
                },
            }],
        },
    ],
    related_formats: &[],
};
