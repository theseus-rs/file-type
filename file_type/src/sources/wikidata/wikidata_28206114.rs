use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206114: FileFormat = FileFormat {
    id: 28_206_114,
    puid: "wikidata/28206114",
    name: "Fuzzy Bitmap",
    extensions: &["cbm", "fbm"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25, 0x62, 0x69, 0x74, 0x6D, 0x61, 0x70])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25, 0x62, 0x69, 0x74, 0x6D, 0x61, 0x70])],
                },
            }],
        },
    ],
    related_formats: &[],
};
