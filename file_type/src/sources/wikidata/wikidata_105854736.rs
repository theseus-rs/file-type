use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854736: FileFormat = FileFormat {
    id: 105_854_736,
    source_type: SourceType::Wikidata,
    name: "CMV compressed data",
    extensions: &["cmv"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x4D, 0x56, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
