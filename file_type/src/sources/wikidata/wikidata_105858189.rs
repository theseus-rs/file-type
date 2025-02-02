use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858189: FileFormat = FileFormat {
    id: 105_858_189,
    source_type: SourceType::Wikidata,
    name: "Elite Japan Crossword Puzzle",
    extensions: &["ejp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x45, 0x4A, 0x50, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
