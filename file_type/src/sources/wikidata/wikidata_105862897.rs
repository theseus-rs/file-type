use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105862897: FileFormat = FileFormat {
    id: 105_862_897,
    source_type: SourceType::Wikidata,
    name: "MiraMon compressed data",
    extensions: &["mmz"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x4D, 0x5A, 0x20, 0x31, 0x2E, 0x30])],
            },
        }],
    }],
    related_formats: &[],
};
