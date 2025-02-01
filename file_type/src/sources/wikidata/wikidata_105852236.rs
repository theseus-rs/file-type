use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852236: FileFormat = FileFormat {
    id: 105_852_236,
    puid: "wikidata/105852236",
    name: "Agon game data archive",
    extensions: &["sfl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x46, 0x4C, 0x31, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
