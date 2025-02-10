use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const WIKIDATA_105851122: FileType = FileType {
    file_format: &FileFormat {
        id: 105_851_122,
        source_type: SourceType::Wikidata,
        name: "ArtCAM Toolpath Template",
        extensions: &["tpl"],
        media_types: &["application/octet-stream"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0xFF, 0xFE, 0xFF, 0x18, 0x41, 0x00, 0x72, 0x00, 0x74, 0x00, 0x43, 0x00,
                        0x41, 0x00, 0x4D, 0x00, 0x20, 0x00, 0x54, 0x00, 0x6F, 0x00, 0x6F, 0x00,
                        0x6C, 0x00, 0x70, 0x00, 0x61, 0x00, 0x74, 0x00, 0x68, 0x00, 0x20, 0x00,
                        0x54, 0x00, 0x65, 0x00, 0x6D, 0x00, 0x70, 0x00, 0x6C, 0x00, 0x61, 0x00,
                        0x74, 0x00, 0x65,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
