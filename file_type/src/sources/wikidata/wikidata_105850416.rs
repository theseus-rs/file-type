use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105850416: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_416,
        source_type: SourceType::Wikidata,
        name: "Compressed File Library 2 compressed data",
        extensions: &["cfl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x46, 0x4C, 0x32])],
                },
            }],
        }],
        related_formats: &[],
    },
};
