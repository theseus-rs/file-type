use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856900: FileFormat = FileFormat {
    id: 105_856_900,
    puid: "wikidata/105856900",
    name: "Ishi Format Go game",
    extensions: &["go", "prb"],
    media_types: &["text/plain", "text/plain"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x56, 0x45, 0x4E, 0x54])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x45, 0x56, 0x45, 0x4E, 0x54])],
                },
            }],
        },
    ],
    related_formats: &[],
};
