use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105862200: FileFormat = FileFormat {
    id: 105_862_200,
    source_type: SourceType::Wikidata,
    name: "Half-life Model",
    extensions: &["mdl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x44, 0x53, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
