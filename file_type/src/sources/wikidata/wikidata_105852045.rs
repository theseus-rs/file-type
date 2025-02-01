use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105852045: FileFormat = FileFormat {
    id: 105_852_045,
    puid: "wikidata/105852045",
    name: "Quake II Save game",
    extensions: &["sav"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7C, 0x03, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
