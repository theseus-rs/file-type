use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858007: FileFormat = FileFormat {
    id: 105_858_007,
    source_type: SourceType::Wikidata,
    name: "Pax Imperia: Eminent Domain game data archive",
    extensions: &["img"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x49, 0x01, 0x00, 0x00, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
