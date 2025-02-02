use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105858803: FileFormat = FileFormat {
    id: 105_858_803,
    source_type: SourceType::Wikidata,
    name: "bitcoin block file",
    extensions: &["dat"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xF9, 0xBE, 0xB4, 0xD9])],
            },
        }],
    }],
    related_formats: &[],
};
