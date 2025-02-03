use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105856197: FileFormat = FileFormat {
    id: 105_856_197,
    source_type: SourceType::Wikidata,
    name: "DR Graph data",
    extensions: &["dat"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x01, 0x30, 0x0F, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
