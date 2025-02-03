use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853732: FileFormat = FileFormat {
    id: 105_853_732,
    source_type: SourceType::Wikidata,
    name: "Any Password data",
    extensions: &["apw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x50])],
            },
        }],
    }],
    related_formats: &[],
};
