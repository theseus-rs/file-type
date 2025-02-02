use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858495: FileFormat = FileFormat {
    id: 105_858_495,
    source_type: SourceType::Wikidata,
    name: "ScriptBasic Binary File Format (64bit)",
    extensions: &["bbf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x38, 0x42, 0x41, 0x53, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
