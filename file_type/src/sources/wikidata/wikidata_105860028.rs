use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105860028: FileType = FileType {
    file_format: &FileFormat {
        id: 105_860_028,
        source_type: SourceType::Wikidata,
        name: "CADVANCE drawing",
        extensions: &["vwf"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x46, 0x20, 0x49, 0x20, 0x54, 0x2C, 0x20, 0x49, 0x6E, 0x63, 0x2E, 0x20,
                        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
                        0x20, 0x20, 0x20, 0x20, 0x0D, 0x0A, 0x43, 0x41, 0x44, 0x56, 0x41, 0x4E,
                        0x43, 0x45, 0x20, 0x64, 0x72, 0x61, 0x77, 0x69, 0x6E, 0x67, 0x20, 0x66,
                        0x69, 0x6C, 0x65, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x20,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
