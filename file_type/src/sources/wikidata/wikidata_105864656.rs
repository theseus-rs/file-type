use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105864656: FileFormat = FileFormat {
    id: 105_864_656,
    source_type: SourceType::Wikidata,
    name: "PassMark PerformanceTest file",
    extensions: &["pt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x54, 0x52, 0x45, 0x53, 0x46, 0x49, 0x4C, 0x45,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
