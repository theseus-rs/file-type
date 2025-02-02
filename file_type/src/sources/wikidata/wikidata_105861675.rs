use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861675: FileFormat = FileFormat {
    id: 105_861_675,
    source_type: SourceType::Wikidata,
    name: "InterBase Layout (v1.0)",
    extensions: &["lot"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x49, 0x6E, 0x74, 0x65, 0x72, 0x42, 0x61, 0x73, 0x65, 0x20, 0x56, 0x31, 0x2C,
                    0x30, 0x30, 0x20, 0x4C, 0x4F, 0x54, 0x0A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
