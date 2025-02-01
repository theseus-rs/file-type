use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855455: FileFormat = FileFormat {
    id: 105_855_455,
    puid: "wikidata/105855455",
    name: "Fanuc parameters file",
    extensions: &["mem", "st1h"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x41, 0x4E, 0x55, 0x43, 0x52, 0x4F, 0x4D,
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
                        0x46, 0x41, 0x4E, 0x55, 0x43, 0x52, 0x4F, 0x4D,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
