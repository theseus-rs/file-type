use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105854308: FileFormat = FileFormat {
    id: 105_854_308,
    source_type: SourceType::Wikidata,
    name: "XelaSoft Archive compressed data",
    extensions: &["xsa"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x50, 0x43, 0x4B])],
            },
        }],
    }],
    related_formats: &[],
};
