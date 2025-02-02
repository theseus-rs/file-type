use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857329: FileFormat = FileFormat {
    id: 105_857_329,
    source_type: SourceType::Wikidata,
    name: "Job Definition Format Job File",
    extensions: &["jdf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
