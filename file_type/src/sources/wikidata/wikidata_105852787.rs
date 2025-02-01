use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852787: FileFormat = FileFormat {
    id: 105_852_787,
    puid: "wikidata/105852787",
    name: "IEEE DASC Standard Delay Format",
    extensions: &["sdf", "sdo"],
    media_types: &["text/plain", "text/plain"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x28, 0x44, 0x45, 0x4C, 0x41, 0x59, 0x46, 0x49, 0x4C, 0x45,
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
                        0x28, 0x44, 0x45, 0x4C, 0x41, 0x59, 0x46, 0x49, 0x4C, 0x45,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
