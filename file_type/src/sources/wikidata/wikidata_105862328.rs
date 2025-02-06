use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862328: FileFormat = FileFormat {
    id: 105_862_328,
    source_type: SourceType::Wikidata,
    name: "Personal Finance Manager Plus data",
    extensions: &["mny"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x46, 0x4D, 0x2E, 0x50, 0x4C, 0x55, 0x53,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
