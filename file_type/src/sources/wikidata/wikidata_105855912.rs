use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855912: FileFormat = FileFormat {
    id: 105_855_912,
    puid: "wikidata/105855912",
    name: "DESI-III drawing",
    extensions: &["bin", "din"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x45, 0x53, 0x49, 0x2D, 0x49, 0x49, 0x49, 0x2D, 0x42, 0x49, 0x4E,
                        0x2D, 0x56,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x45, 0x53, 0x49, 0x2D, 0x49, 0x49, 0x49, 0x2D, 0x42, 0x49, 0x4E,
                        0x2D, 0x56,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[],
};
