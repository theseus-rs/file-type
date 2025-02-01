use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105860081: FileFormat = FileFormat {
    id: 105_860_081,
    puid: "wikidata/105860081",
    name: "Viacom New Media graphics",
    extensions: &["000", "vnm"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x4E, 0x4D, 0x1A])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x56, 0x4E, 0x4D, 0x1A])],
                },
            }],
        },
    ],
    related_formats: &[],
};
