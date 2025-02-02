use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856678: FileFormat = FileFormat {
    id: 105_856_678,
    source_type: SourceType::Wikidata,
    name: "dBASE 5.0 Update",
    extensions: &["upd"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2A, 0x20, 0x64, 0x42, 0x41, 0x53, 0x45, 0x20, 0x35, 0x2E, 0x30, 0x2E, 0x55,
                    0x50, 0x44, 0x20, 0x66, 0x69, 0x6C, 0x65, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
