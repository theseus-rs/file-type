use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_1545782: FileFormat = FileFormat {
    id: 1_545_782,
    source_type: SourceType::Wikidata,
    name: "Smart Game Format",
    extensions: &["sgf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x28, 0x3B])],
            },
        }],
    }],
    related_formats: &[],
};
