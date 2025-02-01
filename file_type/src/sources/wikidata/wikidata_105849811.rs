use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849811: FileFormat = FileFormat {
    id: 105_849_811,
    puid: "wikidata/105849811",
    name: "Wii Color Swapping Animation",
    extensions: &["clr0"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x4C, 0x52, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
