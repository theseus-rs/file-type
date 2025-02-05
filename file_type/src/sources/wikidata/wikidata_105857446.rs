use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857446: FileFormat = FileFormat {
    id: 105_857_446,
    source_type: SourceType::Wikidata,
    name: "Wikireader Forth program",
    extensions: &["4mu"],
    media_types: &["text/plain"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x69, 0x6E, 0x63, 0x6C, 0x75, 0x64, 0x65, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
