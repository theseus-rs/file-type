use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105857416: FileFormat = FileFormat {
    id: 105_857_416,
    source_type: SourceType::Wikidata,
    name: "JCALG1 compressed data",
    extensions: &["jc"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4A, 0x43])],
            },
        }],
    }],
    related_formats: &[],
};
