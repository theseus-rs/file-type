use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105856209: FileFormat = FileFormat {
    id: 105_856_209,
    puid: "wikidata/105856209",
    name: "DeleD map",
    extensions: &["dmf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x65, 0x6C, 0x65, 0x44, 0x20, 0x4D, 0x61, 0x70, 0x20, 0x46, 0x69, 0x6C,
                    0x65, 0x3B,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
