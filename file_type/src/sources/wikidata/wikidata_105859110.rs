use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105859110: FileType = FileType {
    file_format: &FileFormat {
        id: 105_859_110,
        source_type: SourceType::Wikidata,
        name: "WinJack for MS Win game variant (v1.x)",
        extensions: &["bj"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x57, 0x69, 0x6E, 0x4A, 0x61, 0x63, 0x6B, 0x20, 0x66, 0x6F, 0x72, 0x20,
                        0x4D, 0x69, 0x63, 0x72, 0x6F, 0x53, 0x6F, 0x66, 0x74, 0x20, 0x57, 0x69,
                        0x6E, 0x64, 0x6F, 0x77, 0x73, 0x2E, 0x20, 0x56, 0x65, 0x72, 0x2E, 0x20,
                        0x31, 0x2E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
