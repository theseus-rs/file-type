use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105851668: FileFormat = FileFormat {
    id: 105_851_668,
    source_type: SourceType::Wikidata,
    name: "Sublime Text Snippets",
    extensions: &["sublime-snippet"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x3C, 0x73, 0x6E, 0x69, 0x70, 0x70, 0x65, 0x74, 0x3E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
