use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851329: FileFormat = FileFormat {
    id: 105_851_329,
    puid: "wikidata/105851329",
    name: "Ports of Call savegame",
    extensions: &["trp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x44, 0x4B, 0x20, 0x54, 0x52, 0x41, 0x4D, 0x50, 0x20, 0x31, 0x2E, 0x30,
                    0x20, 0x28, 0x43, 0x29, 0x20, 0x31, 0x39, 0x38, 0x37, 0x2C, 0x4D, 0x75, 0x65,
                    0x6E, 0x63, 0x68, 0x65, 0x6E, 0x00, 0x1A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
