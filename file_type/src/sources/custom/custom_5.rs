use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const CUSTOM_5: FileFormat = FileFormat {
    id: 5,
    source_type: SourceType::Custom,
    name: "Apache Parquet",
    extensions: &["parquet"],
    media_types: &["application/vnd.apache.parquet"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x41, 0x52, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
