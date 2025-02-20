use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105867060: FileType = FileType {
    file_format: &FileFormat {
        id: 105_867_060,
        source_type: SourceType::Wikidata,
        name: "Daisy-Dot NLQ font",
        extensions: &["nlq"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x44, 0x41, 0x49, 0x53, 0x59, 0x2D, 0x44, 0x4F, 0x54, 0x20, 0x4E, 0x4C,
                        0x51, 0x20, 0x46, 0x4F, 0x4E, 0x54,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
