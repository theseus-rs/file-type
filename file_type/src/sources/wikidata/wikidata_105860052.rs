use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105860052: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_052,
        source_type: SourceType::Wikidata,
        name: "Vividas video",
        extensions: &["viv"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x76, 0x69, 0x76, 0x69, 0x64, 0x61, 0x73])],
                },
            }],
        }],
        related_formats: &[],
    },
};
