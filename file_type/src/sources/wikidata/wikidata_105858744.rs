use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858744: FileFormat = FileFormat {
    id: 105_858_744,
    puid: "wikidata/105858744",
    name: "Cloe picture bitmap (big endian)",
    extensions: &["clo", "cloe"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x4C, 0x4F, 0x45, 0x49, 0x49])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x4C, 0x4F, 0x45, 0x49, 0x49])],
                },
            }],
        },
    ],
    related_formats: &[],
};
