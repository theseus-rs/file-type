use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105853667: FileFormat = FileFormat {
    id: 105_853_667,
    source_type: SourceType::Wikidata,
    name: "Advanced Digital Audio lossless compressed audio",
    extensions: &["ada"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x44, 0x41, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
