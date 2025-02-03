use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105860496: FileFormat = FileFormat {
    id: 105_860_496,
    source_type: SourceType::Wikidata,
    name: "GisRX GPS Navigator map",
    extensions: &["rxm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x52, 0x53, 0x41])],
            },
        }],
    }],
    related_formats: &[],
};
