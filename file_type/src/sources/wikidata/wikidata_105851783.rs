use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105851783: FileFormat = FileFormat {
    id: 105_851_783,
    puid: "wikidata/105851783",
    name: "K-Spreadsheet (v2)",
    extensions: &["spd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x6C, 0x69, 0x66, 0x66, 0x20, 0x48, 0x41, 0x52, 0x4B, 0x45, 0x52, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
