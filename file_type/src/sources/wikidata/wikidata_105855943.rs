use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105855943: FileFormat = FileFormat {
    id: 105_855_943,
    puid: "wikidata/105855943",
    name: "Kyle game data container",
    extensions: &["dta"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4B, 0x79, 0x6C, 0x65, 0x20, 0x44, 0x54, 0x41,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
