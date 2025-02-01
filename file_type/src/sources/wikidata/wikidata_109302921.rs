use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_109302921: FileFormat = FileFormat {
    id: 109_302_921,
    puid: "wikidata/109302921",
    name: "Asymetrix Compel Presentation, version 1",
    extensions: &["cpl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x03, 0x4A, 0x42, 0x4F, 0x4E, 0xD3, 0x2C, 0x41,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
