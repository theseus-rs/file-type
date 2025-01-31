use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861366: FileFormat = FileFormat {
    id: 105_861_366,
    puid: "wikidata/105861366",
    name: "MicroLathe object",
    extensions: &["lat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xE6, 0x4C, 0x41, 0x54, 0x48, 0x45])],
            },
        }],
    }],
    related_formats: &[],
};
