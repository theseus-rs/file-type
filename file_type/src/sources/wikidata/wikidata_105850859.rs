use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105850859: FileFormat = FileFormat {
    id: 105_850_859,
    puid: "wikidata/105850859",
    name: "Kurzweil K2-serie sample",
    extensions: &["kr1", "krz"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x52, 0x41, 0x4D, 0x00, 0x00, 0x00])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x52, 0x41, 0x4D, 0x00, 0x00, 0x00])],
                },
            }],
        },
    ],
    related_formats: &[],
};
