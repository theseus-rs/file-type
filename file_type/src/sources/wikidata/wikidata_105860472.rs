use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860472: FileFormat = FileFormat {
    id: 105_860_472,
    source_type: SourceType::Wikidata,
    name: "REALbasic/Xojo Project",
    extensions: &["rbp"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x62, 0x42, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
