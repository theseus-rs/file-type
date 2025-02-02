use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852551: FileFormat = FileFormat {
    id: 105_852_551,
    source_type: SourceType::Wikidata,
    name: "Bohemian Coding Sketch drawing (SQLite based)",
    extensions: &["sketch"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x53, 0x51, 0x4C, 0x69, 0x74, 0x65, 0x20, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74,
                    0x20, 0x33,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
