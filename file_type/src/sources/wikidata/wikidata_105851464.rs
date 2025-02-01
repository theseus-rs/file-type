use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851464: FileFormat = FileFormat {
    id: 105_851_464,
    puid: "wikidata/105851464",
    name: "Klasik Table/spreadsheet",
    extensions: &["tab"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4B, 0x6C, 0x61, 0x73, 0x69, 0x6B, 0x20, 0x74, 0x61, 0x62, 0x75, 0x6C, 0x6B,
                    0x61, 0x0D,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
