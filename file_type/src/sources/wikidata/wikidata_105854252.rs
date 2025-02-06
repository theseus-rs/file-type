use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854252: FileFormat = FileFormat {
    id: 105_854_252,
    source_type: SourceType::Wikidata,
    name: "LZGT compressed data",
    extensions: &["lzt"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x5A, 0x54, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
