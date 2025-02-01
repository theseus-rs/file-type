use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859189: FileFormat = FileFormat {
    id: 105_859_189,
    puid: "wikidata/105859189",
    name: "HP ASII GROB bitmap (embedded)",
    extensions: &["asc", "grb", "gro"],
    media_types: &["text/plain", "text/plain", "text/plain"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25, 0x25, 0x48, 0x50, 0x3A, 0x20])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25, 0x25, 0x48, 0x50, 0x3A, 0x20])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x25, 0x25, 0x48, 0x50, 0x3A, 0x20])],
                },
            }],
        },
    ],
    related_formats: &[],
};
