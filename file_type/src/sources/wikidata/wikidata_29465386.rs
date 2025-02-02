use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_29465386: FileFormat = FileFormat {
    id: 29_465_386,
    source_type: SourceType::Wikidata,
    name: "UltraEdit wordfile",
    extensions: &["uew"],
    media_types: &["text/plain"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2F, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
