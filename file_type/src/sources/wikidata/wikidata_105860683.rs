use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860683: FileFormat = FileFormat {
    id: 105_860_683,
    source_type: SourceType::Wikidata,
    name: "REAPER media peak information (v1.0)",
    extensions: &["reapeaks"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x50, 0x4B, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
