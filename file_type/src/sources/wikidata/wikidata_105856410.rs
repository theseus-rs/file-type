use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105856410: FileFormat = FileFormat {
    id: 105_856_410,
    source_type: SourceType::Wikidata,
    name: "War Thunder replay",
    extensions: &["wrpl"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0xE5, 0xAC, 0x00, 0x10, 0xE7, 0x88, 0x01, 0x00, 0x6C, 0x65, 0x76, 0x65, 0x6C,
                    0x73, 0x2F,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
