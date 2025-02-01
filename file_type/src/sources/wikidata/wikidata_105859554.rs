use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859554: FileFormat = FileFormat {
    id: 105_859_554,
    puid: "wikidata/105859554",
    name: "CRYO UBB video",
    extensions: &["hnm", "ubb"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0x42, 0x42, 0x32])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x55, 0x42, 0x42, 0x32])],
                },
            }],
        },
    ],
    related_formats: &[],
};
