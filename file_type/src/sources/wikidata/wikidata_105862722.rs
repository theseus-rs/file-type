use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862722: FileFormat = FileFormat {
    id: 105_862_722,
    source_type: SourceType::Wikidata,
    name: "Leges Motus Map",
    extensions: &["map"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x6E, 0x61, 0x6D, 0x65, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
