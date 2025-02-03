use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860991: FileFormat = FileFormat {
    id: 105_860_991,
    source_type: SourceType::Wikidata,
    name: "Logomotion Graphic File",
    extensions: &["lgf"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4C, 0x47, 0x46, 0x30, 0x34, 0x01, 0x00, 0x00, 0x00, 0x49, 0x4D, 0x5A, 0x31,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
