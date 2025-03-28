use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105857456: FileType = FileType {
    file_format: &FileFormat {
        id: 105_857_456,
        source_type: SourceType::Wikidata,
        name: "PathMinder encrypted (v4.00)",
        extensions: &["000"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x69, 0x6C, 0x65, 0x20, 0x65, 0x6E, 0x63, 0x72, 0x79, 0x70, 0x74,
                        0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x50, 0x61, 0x74, 0x68, 0x4D, 0x69,
                        0x6E, 0x64, 0x65, 0x72, 0x20, 0x76, 0x34, 0x2E, 0x30, 0x30, 0x20, 0x28,
                        0x63, 0x29, 0x20, 0x43, 0x6F, 0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74,
                        0x20, 0x31, 0x39, 0x38, 0x34, 0x2C, 0x31, 0x39, 0x38, 0x35, 0x20, 0x57,
                        0x65, 0x73, 0x74, 0x6C, 0x61, 0x6B, 0x65, 0x20, 0x44, 0x61, 0x74, 0x61,
                        0x20, 0x43, 0x6F, 0x72, 0x70, 0x6F, 0x72, 0x61, 0x74, 0x69, 0x6F, 0x6E,
                        0x0D, 0x0A, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
