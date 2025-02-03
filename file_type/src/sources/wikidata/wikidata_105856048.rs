use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856048: FileFormat = FileFormat {
    id: 105_856_048,
    source_type: SourceType::Wikidata,
    name: "LocoScript Document (v1.x)",
    extensions: &["doc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x4F, 0x43, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
