use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105855496: FileFormat = FileFormat {
    id: 105_855_496,
    source_type: SourceType::Wikidata,
    name: "dBASE IV Form design",
    extensions: &["frm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x64, 0x42, 0x41, 0x53, 0x45, 0x20, 0x49, 0x56, 0x20, 0x47, 0x65, 0x6E, 0x65,
                    0x72, 0x69, 0x63, 0x20, 0x44, 0x65, 0x73, 0x69, 0x67, 0x6E, 0x20, 0x46, 0x69,
                    0x6C, 0x65, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20, 0x31, 0x2E,
                    0x30, 0x00, 0x01, 0x07, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
