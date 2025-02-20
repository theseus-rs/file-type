use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105858410: FileType = FileType {
    file_format: &FileFormat {
        id: 105_858_410,
        source_type: SourceType::Wikidata,
        name: "COM-Pack library",
        extensions: &["com"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xE9, 0x46, 0x01, 0x0A, 0x0D, 0x20, 0x20, 0x43, 0x4F, 0x4D, 0x2D, 0x50,
                        0x61, 0x63, 0x6B, 0x20, 0x76, 0x31, 0x2E, 0x30, 0x20, 0xFE, 0x20, 0x43,
                        0x6F, 0x70, 0x72, 0x2E, 0x20, 0x31, 0x39, 0x38, 0x39, 0x20, 0x62, 0x79,
                        0x20, 0x4A, 0x61, 0x63, 0x6B, 0x20, 0x41, 0x2E, 0x20, 0x4F, 0x72, 0x6D,
                        0x61, 0x6E, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x0A, 0x0D,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
