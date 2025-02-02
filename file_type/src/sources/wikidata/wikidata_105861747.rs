use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105861747: FileFormat = FileFormat {
    id: 105_861_747,
    source_type: SourceType::Wikidata,
    name: "Exotic AdLib module",
    extensions: &["xad"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x41, 0x44, 0x21])],
            },
        }],
    }],
    related_formats: &[],
};
