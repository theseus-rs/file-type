use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105866060: FileFormat = FileFormat {
    id: 105_866_060,
    puid: "wikidata/105866060",
    name: "Protracker Studio 16 module/song",
    extensions: &["ps16", "psm"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x53, 0x31, 0x36, 0xFE])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x50, 0x53, 0x31, 0x36, 0xFE])],
                },
            }],
        },
    ],
    related_formats: &[],
};
