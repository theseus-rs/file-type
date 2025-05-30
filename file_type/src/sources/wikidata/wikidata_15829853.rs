use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_15829853: FileType = FileType {
    file_format: &FileFormat {
        id: 15_829_853,
        source_type: SourceType::Wikidata,
        name: "QGIS layer style",
        extensions: &["qml"],
        media_types: &["test/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x21, 0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45, 0x20, 0x71, 0x67,
                        0x69, 0x73, 0x20, 0x50, 0x55, 0x42, 0x4C, 0x49, 0x43, 0x20, 0x27, 0x68,
                        0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x6D, 0x72, 0x63, 0x63, 0x2E, 0x63,
                        0x6F, 0x6D, 0x2F, 0x71, 0x67, 0x69, 0x73, 0x2E, 0x64, 0x74, 0x64, 0x27,
                        0x20, 0x27, 0x53, 0x59, 0x53, 0x54, 0x45, 0x4D, 0x27, 0x3E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
