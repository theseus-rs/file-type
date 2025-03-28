use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2661: FileType = FileType {
    file_format: &FileFormat {
        id: 2_661,
        source_type: SourceType::Pronom,
        name: "Raw PIMA SWIR Reflectance Spectral File",
        extensions: &["fos"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x53, 0x50, 0x4C, 0x46, 0x4F, 0x53, 0x00, 0x01, 0x05, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
