use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105861899: FileType = FileType {
    file_format: &FileFormat {
        id: 105_861_899,
        source_type: SourceType::Wikidata,
        name: "Remote Assistance Request",
        extensions: &["msrcincident"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xFE, 0x3C, 0x00, 0x3F, 0x00, 0x78, 0x00, 0x6D, 0x00, 0x6C, 0x00,
                        0x20, 0x00, 0x76, 0x00, 0x65, 0x00, 0x72, 0x00, 0x73, 0x00, 0x69, 0x00,
                        0x6F, 0x00, 0x6E, 0x00, 0x3D, 0x00, 0x22, 0x00, 0x31, 0x00, 0x2E, 0x00,
                        0x30, 0x00, 0x22, 0x00, 0x20, 0x00, 0x65, 0x00, 0x6E, 0x00, 0x63, 0x00,
                        0x6F, 0x00, 0x64, 0x00, 0x69, 0x00, 0x6E, 0x00, 0x67, 0x00, 0x3D, 0x00,
                        0x22, 0x00, 0x55, 0x00, 0x6E, 0x00, 0x69, 0x00, 0x63, 0x00, 0x6F, 0x00,
                        0x64, 0x00, 0x65, 0x00, 0x22, 0x00, 0x20, 0x00, 0x3F, 0x00, 0x3E, 0x00,
                        0x3C, 0x00, 0x55, 0x00, 0x50, 0x00, 0x4C, 0x00, 0x4F, 0x00, 0x41, 0x00,
                        0x44, 0x00, 0x49, 0x00, 0x4E, 0x00, 0x46, 0x00, 0x4F, 0x00, 0x20, 0x00,
                        0x54, 0x00, 0x59, 0x00, 0x50, 0x00, 0x45, 0x00, 0x3D, 0x00, 0x22, 0x00,
                        0x45, 0x00, 0x73, 0x00, 0x63, 0x00, 0x61, 0x00, 0x6C, 0x00, 0x61, 0x00,
                        0x74, 0x00, 0x65, 0x00, 0x64, 0x00, 0x22, 0x00, 0x3E, 0x00, 0x3C, 0x00,
                        0x55, 0x00, 0x50, 0x00, 0x4C, 0x00, 0x4F, 0x00, 0x41, 0x00, 0x44, 0x00,
                        0x44, 0x00, 0x41, 0x00, 0x54, 0x00, 0x41, 0x00, 0x20, 0x00, 0x55, 0x00,
                        0x53, 0x00, 0x45, 0x00, 0x52, 0x00, 0x4E, 0x00, 0x41, 0x00, 0x4D, 0x00,
                        0x45, 0x00, 0x3D, 0x00, 0x22, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
