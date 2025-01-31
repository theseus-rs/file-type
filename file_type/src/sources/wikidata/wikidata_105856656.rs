use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856656: FileFormat = FileFormat {
    id: 105_856_656,
    puid: "wikidata/105856656",
    name: "id Software's DOOM Patch-WAD",
    extensions: &["wad"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x57, 0x41, 0x44])],
            },
        }],
    }],
    related_formats: &[],
};
