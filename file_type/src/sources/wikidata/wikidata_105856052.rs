use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856052: FileFormat = FileFormat {
    id: 105_856_052,
    source_type: SourceType::Wikidata,
    name: "DFF format (v3.0, BE)",
    extensions: &["dff"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x44, 0x46, 0x33])],
            },
        }],
    }],
    related_formats: &[],
};
