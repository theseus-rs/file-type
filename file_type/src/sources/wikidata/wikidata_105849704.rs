use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105849704: FileFormat = FileFormat {
    id: 105_849_704,
    source_type: SourceType::Wikidata,
    name: "BoomTracker 4.0 module",
    extensions: &["cff"],
    media_types: &[],
    signatures: &[Signature {
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
