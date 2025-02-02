use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857246: FileFormat = FileFormat {
    id: 105_857_246,
    source_type: SourceType::Wikidata,
    name: "MaxonMAGIC Sound sample (v1.0)",
    extensions: &["hsn"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x48, 0x53, 0x4E, 0x44, 0x31, 0x2E, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
