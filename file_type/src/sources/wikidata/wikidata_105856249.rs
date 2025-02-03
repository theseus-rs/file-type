use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856249: FileFormat = FileFormat {
    id: 105_856_249,
    source_type: SourceType::Wikidata,
    name: "Delusion samples library",
    extensions: &["del"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x45, 0x4C, 0x55])],
            },
        }],
    }],
    related_formats: &[],
};
