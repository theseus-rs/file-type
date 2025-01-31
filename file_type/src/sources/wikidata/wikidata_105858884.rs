use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858884: FileFormat = FileFormat {
    id: 105_858_884,
    puid: "wikidata/105858884",
    name: "BX Embrilliance font",
    extensions: &["bx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x42, 0x58, 0x30, 0x30, 0x31, 0x49, 0x44, 0x4D, 0x44, 0x54, 0x4C,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
