use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852428: FileFormat = FileFormat {
    id: 105_852_428,
    source_type: SourceType::Wikidata,
    name: "Sonnet Input Data",
    extensions: &["sid"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x21, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
