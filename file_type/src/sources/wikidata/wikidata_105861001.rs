use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861001: FileFormat = FileFormat {
    id: 105_861_001,
    puid: "wikidata/105861001",
    name: "Lua 5.2 bytecode",
    extensions: &["out"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1B, 0x4C, 0x75, 0x61, 0x52])],
            },
        }],
    }],
    related_formats: &[],
};
