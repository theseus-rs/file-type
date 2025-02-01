use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856618: FileFormat = FileFormat {
    id: 105_856_618,
    puid: "wikidata/105856618",
    name: "Fugawi 3 waypoint format",
    extensions: &["wpt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x46, 0x33, 0x57, 0x50, 0x54, 0x2D, 0x2D, 0x2E, 0x2D, 0x2D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
