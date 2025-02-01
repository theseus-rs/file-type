use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854157: FileFormat = FileFormat {
    id: 105_854_157,
    puid: "wikidata/105854157",
    name: "ar archive (thin)",
    extensions: &["a", "ar", "lbr"],
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
                    tokens: &[Token::Literal(&[0x21, 0x3C, 0x74, 0x68, 0x69, 0x6E, 0x3E])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x21, 0x3C, 0x74, 0x68, 0x69, 0x6E, 0x3E])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x21, 0x3C, 0x74, 0x68, 0x69, 0x6E, 0x3E])],
                },
            }],
        },
    ],
    related_formats: &[],
};
