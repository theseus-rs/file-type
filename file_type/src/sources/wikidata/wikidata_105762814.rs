use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105762814: FileType = FileType {
    file_format: &FileFormat {
        id: 105_762_814,
        source_type: SourceType::Wikidata,
        name: "AS SSD Benchmark results",
        extensions: &[],
        media_types: &["text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xEF, 0xBB, 0xBF, 0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72,
                        0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x20, 0x65,
                        0x6E, 0x63, 0x6F, 0x64, 0x69, 0x6E, 0x67, 0x3D, 0x22, 0x75, 0x74, 0x66,
                        0x2D, 0x38, 0x22, 0x3F, 0x3E, 0x0D, 0x0A, 0x3C, 0x41, 0x53, 0x53, 0x53,
                        0x44, 0x42, 0x65, 0x6E, 0x63, 0x68, 0x6D, 0x61, 0x72, 0x6B, 0x0D, 0x0A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
