use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105852668: FileFormat = FileFormat {
    id: 105_852_668,
    source_type: SourceType::Wikidata,
    name: "Scala Multimedia Script (v2.0)",
    extensions: &["script"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x56, 0x32, 0x2E, 0x30, 0x0A, 0x0A])],
            },
        }],
    }],
    related_formats: &[],
};
