use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105850214: FileType = FileType {
    file_format: &FileFormat {
        id: 105_850_214,
        source_type: SourceType::Wikidata,
        name: "16bit DOS COM FDS-CP protected",
        extensions: &["com"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x8C, 0xCA, 0x2E, 0x89, 0x16, 0xCD, 0xEB, 0xB4, 0x30, 0x8B, 0x2E, 0x02,
                        0x00, 0x8B, 0x1E, 0x2C, 0x00, 0x8E, 0xDA, 0xA3, 0x7F, 0xEA, 0x8C, 0x06,
                        0x89, 0xEA, 0x89, 0x1E, 0x93, 0xEA, 0x89, 0x2E, 0x9D, 0xEA, 0xEB, 0x18,
                        0x0D, 0x0A, 0x0D, 0x0A, 0x20, 0x28, 0x63, 0x29, 0x20, 0x66, 0x64, 0x73,
                        0x30, 0x66, 0x74, 0x0D, 0x0A, 0x0D, 0x0A, 0x1A,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
