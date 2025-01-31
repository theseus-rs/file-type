use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861693: FileFormat = FileFormat {
    id: 105_861_693,
    puid: "wikidata/105861693",
    name: "NASCAR SimRacing game data archive",
    extensions: &["mas"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x55, 0x42, 0x45, 0x4D, 0x41, 0x53, 0x34, 0x2E, 0x31, 0x30, 0x00, 0x00,
                    0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
