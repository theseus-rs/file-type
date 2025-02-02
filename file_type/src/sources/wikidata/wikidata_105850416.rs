use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105850416: FileFormat = FileFormat {
    id: 105_850_416,
    source_type: SourceType::Wikidata,
    name: "Compressed File Library 2 compressed data",
    extensions: &["cfl"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x46, 0x4C, 0x32])],
            },
        }],
    }],
    related_formats: &[],
};
