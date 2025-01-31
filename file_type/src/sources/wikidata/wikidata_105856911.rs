use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856911: FileFormat = FileFormat {
    id: 105_856_911,
    puid: "wikidata/105856911",
    name: "Crossword Puzzle",
    extensions: &["gpm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x1A, 0x32, 0x30, 0x43, 0x57, 0x50, 0x55, 0x5A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
