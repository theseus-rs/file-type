use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858693: FileFormat = FileFormat {
    id: 105_858_693,
    source_type: SourceType::Wikidata,
    name: "ScriptBasic Binary File Format (32bit)",
    extensions: &["bbf"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x34, 0x42, 0x41, 0x53, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
