use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851270: FileFormat = FileFormat {
    id: 105_851_270,
    puid: "wikidata/105851270",
    name: "tzip compressed file",
    extensions: &["tz", "tzip"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x5A, 0x00])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x5A, 0x00])],
                },
            }],
        },
    ],
    related_formats: &[],
};
