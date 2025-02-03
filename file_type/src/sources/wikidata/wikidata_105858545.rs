use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858545: FileFormat = FileFormat {
    id: 105_858_545,
    source_type: SourceType::Wikidata,
    name: "CHDK UBASIC script",
    extensions: &["bas"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x40, 0x74, 0x69, 0x74, 0x6C, 0x65, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
