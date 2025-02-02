use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856331: FileFormat = FileFormat {
    id: 105_856_331,
    source_type: SourceType::Wikidata,
    name: "Lotus Works Document",
    extensions: &["doc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xAE, 0x26, 0x56, 0x30, 0x30, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
