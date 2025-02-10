use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105854234: FileType = FileType {
    file_format: &FileFormat {
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
    },
};
