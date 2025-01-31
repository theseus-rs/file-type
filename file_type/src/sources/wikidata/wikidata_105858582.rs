use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858582: FileFormat = FileFormat {
    id: 105_858_582,
    puid: "wikidata/105858582",
    name: "Cloe picture bitmap (little endian)",
    extensions: &["clo", "cloe"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x4C, 0x4F, 0x45, 0x4D, 0x4D])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x4C, 0x4F, 0x45, 0x4D, 0x4D])],
                },
            }],
        },
    ],
    related_formats: &[],
};
