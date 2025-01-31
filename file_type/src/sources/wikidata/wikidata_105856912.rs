use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856912: FileFormat = FileFormat {
    id: 105_856_912,
    puid: "wikidata/105856912",
    name: "Airline Tycoon game data archive",
    extensions: &["gli", "glj"],
    media_types: &["application/octet-stream", "application/octet-stream"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x4C, 0x49, 0x42, 0x32])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x47, 0x4C, 0x49, 0x42, 0x32])],
                },
            }],
        },
    ],
    related_formats: &[],
};
