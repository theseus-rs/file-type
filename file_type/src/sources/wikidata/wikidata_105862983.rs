use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862983: FileFormat = FileFormat {
    id: 105_862_983,
    source_type: SourceType::Wikidata,
    name: "Delta Music module (variant)",
    extensions: &["dlm"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x4C, 0x54, 0x41, 0x54, 0x52, 0x41, 0x43,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
