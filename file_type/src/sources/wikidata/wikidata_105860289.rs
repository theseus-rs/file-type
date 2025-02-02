use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860289: FileFormat = FileFormat {
    id: 105_860_289,
    source_type: SourceType::Wikidata,
    name: "Ultimate Stunts Replay",
    extensions: &["repl"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x55, 0x6C, 0x74, 0x69, 0x6D, 0x61, 0x74, 0x65, 0x20, 0x53, 0x74, 0x75, 0x6E,
                    0x74, 0x73, 0x20, 0x72, 0x65, 0x70, 0x6C, 0x61, 0x79, 0x20, 0x66, 0x6F, 0x72,
                    0x6D, 0x61, 0x74, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
