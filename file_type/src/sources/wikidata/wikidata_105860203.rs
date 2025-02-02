use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860203: FileFormat = FileFormat {
    id: 105_860_203,
    source_type: SourceType::Wikidata,
    name: "Age of Conan game data archive",
    extensions: &["rdbdata"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x44, 0x42, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
