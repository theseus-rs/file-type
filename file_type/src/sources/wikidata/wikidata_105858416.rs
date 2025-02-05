use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858416: FileFormat = FileFormat {
    id: 105_858_416,
    source_type: SourceType::Wikidata,
    name: "Turbo Lightning Environment",
    extensions: &["env"],
    media_types: &["application/octet-stream"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x07, 0x2A, 0x54, 0x4C, 0x45, 0x4E, 0x56, 0x2A,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
