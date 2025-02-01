use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105854319: FileFormat = FileFormat {
    id: 105_854_319,
    puid: "wikidata/105854319",
    name: "Top 4 compressed data",
    extensions: &["t4", "to4"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x34, 0x1A, 0x00])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x34, 0x1A, 0x00])],
                },
            }],
        },
    ],
    related_formats: &[],
};
