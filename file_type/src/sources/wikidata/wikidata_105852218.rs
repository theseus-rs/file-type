use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852218: FileFormat = FileFormat {
    id: 105_852_218,
    source_type: SourceType::Wikidata,
    name: "Duxbury Scrub Table",
    extensions: &["sbt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7C, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
