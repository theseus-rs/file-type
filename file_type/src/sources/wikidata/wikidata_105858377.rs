use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858377: FileFormat = FileFormat {
    id: 105_858_377,
    source_type: SourceType::Wikidata,
    name: "Organize! Environment",
    extensions: &["env"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xD3, 0xD0])],
            },
        }],
    }],
    related_formats: &[],
};
