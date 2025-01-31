use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105849639: FileFormat = FileFormat {
    id: 105_849_639,
    puid: "wikidata/105849639",
    name: "The Print Shop Deluxe Card Layout",
    extensions: &["cly"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x53, 0x44, 0x65, 0x6C, 0x75, 0x78, 0x65, 0x2E, 0x43, 0x4C, 0x59,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
