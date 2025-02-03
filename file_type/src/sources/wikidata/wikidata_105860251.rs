use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860251: FileFormat = FileFormat {
    id: 105_860_251,
    source_type: SourceType::Wikidata,
    name: "Sierra Robot Animation",
    extensions: &["rbt"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x16, 0x00, 0x53, 0x4F, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
