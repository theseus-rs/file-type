use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856010: FileFormat = FileFormat {
    id: 105_856_010,
    source_type: SourceType::Wikidata,
    name: "L3DT Design Map File",
    extensions: &["dmf"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x33, 0x44, 0x54, 0xC8, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
