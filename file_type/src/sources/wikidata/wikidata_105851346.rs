use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851346: FileFormat = FileFormat {
    id: 105_851_346,
    source_type: SourceType::Wikidata,
    name: "Text - UTF-16 (LE) encoded",
    extensions: &["txt"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0xFE])],
            },
        }],
    }],
    related_formats: &[],
};
