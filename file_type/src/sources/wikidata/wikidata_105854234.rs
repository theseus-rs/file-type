use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105854234: FileFormat = FileFormat {
    id: 105_854_234,
    source_type: SourceType::Wikidata,
    name: "WSL compressed data",
    extensions: &["wsl"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x57, 0x53, 0x4C, 0x3E])],
            },
        }],
    }],
    related_formats: &[],
};
