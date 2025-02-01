use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860077: FileFormat = FileFormat {
    id: 105_860_077,
    puid: "wikidata/105860077",
    name: "CRYO HNM6 video",
    extensions: &["hnm", "hns"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x4E, 0x4D, 0x36])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x48, 0x4E, 0x4D, 0x36])],
                },
            }],
        },
    ],
    related_formats: &[],
};
