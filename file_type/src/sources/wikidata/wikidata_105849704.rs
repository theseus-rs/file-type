use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849704: FileFormat = FileFormat {
    id: 105_849_704,
    puid: "wikidata/105849704",
    name: "BoomTracker 4.0 module",
    extensions: &["cff"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x43, 0x55, 0x44, 0x2D, 0x46, 0x4D, 0x2D, 0x46, 0x69, 0x6C, 0x65, 0x3E,
                    0x1A, 0xDE, 0xE0, 0x01,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
