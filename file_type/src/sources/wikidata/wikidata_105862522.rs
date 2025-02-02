use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862522: FileFormat = FileFormat {
    id: 105_862_522,
    source_type: SourceType::Wikidata,
    name: "The Player 5.0a module",
    extensions: &["p50"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x35, 0x30, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
