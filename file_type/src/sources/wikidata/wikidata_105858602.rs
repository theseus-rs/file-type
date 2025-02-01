use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858602: FileFormat = FileFormat {
    id: 105_858_602,
    puid: "wikidata/105858602",
    name: "HSI JPEG bitmap",
    extensions: &["hsi", "jpg"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x68, 0x73, 0x69, 0x31])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x68, 0x73, 0x69, 0x31])],
                },
            }],
        },
    ],
    related_formats: &[],
};
