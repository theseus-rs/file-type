use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857342: FileFormat = FileFormat {
    id: 105_857_342,
    source_type: SourceType::Wikidata,
    name: "JRun Server Application",
    extensions: &["jsa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xA2, 0xAB, 0x0B, 0xF0, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
