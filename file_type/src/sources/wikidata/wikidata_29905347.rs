use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29905347: FileFormat = FileFormat {
    id: 29_905_347,
    puid: "wikidata/29905347",
    name: "Simple Highspeed Archiver",
    extensions: &["sharc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1B, 0xF3, 0xB9, 0x71])],
            },
        }],
    }],
    related_formats: &[],
};
