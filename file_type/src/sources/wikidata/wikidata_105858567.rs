use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858567: FileFormat = FileFormat {
    id: 105_858_567,
    source_type: SourceType::Wikidata,
    name: "C64 Hires bitmap",
    extensions: &["hir"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
