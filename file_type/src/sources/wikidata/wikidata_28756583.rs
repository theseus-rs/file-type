use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28756583: FileFormat = FileFormat {
    id: 28_756_583,
    puid: "wikidata/28756583",
    name: "Fountain script",
    extensions: &["fountain", "spmd"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x69, 0x74, 0x6C, 0x65, 0x3A])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x69, 0x74, 0x6C, 0x65, 0x3A])],
                },
            }],
        },
    ],
    related_formats: &[],
};
