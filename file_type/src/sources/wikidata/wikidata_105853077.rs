use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853077: FileFormat = FileFormat {
    id: 105_853_077,
    source_type: SourceType::Wikidata,
    name: "phpMyAdmin SQL dump",
    extensions: &["sql"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x2D, 0x2D, 0x20, 0x70, 0x68, 0x70, 0x4D, 0x79, 0x41, 0x64, 0x6D, 0x69, 0x6E,
                    0x20, 0x53, 0x51, 0x4C, 0x20, 0x44, 0x75, 0x6D, 0x70,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
