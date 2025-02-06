use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105855106: FileFormat = FileFormat {
    id: 105_855_106,
    source_type: SourceType::Wikidata,
    name: "AxCrypt encrypted",
    extensions: &["axx"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xC0, 0xB9, 0x07, 0x2E, 0x4F, 0x93, 0xF1, 0x46, 0xA0, 0x15, 0x79, 0x2C, 0xA1,
                    0xD9, 0xE8, 0x21, 0x15, 0x00, 0x00, 0x00, 0x02,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
