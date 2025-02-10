use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_2388: FileType = FileType {
    file_format: &FileFormat {
        id: 2_388,
        source_type: SourceType::Pronom,
        name: "ERDAS Imagine Large Raster Spill File",
        extensions: &["ige"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x45, 0x52, 0x44, 0x41, 0x53, 0x5F, 0x49, 0x4D, 0x47, 0x5F, 0x45, 0x58,
                        0x54, 0x45, 0x52, 0x4E, 0x41, 0x4C, 0x5F, 0x52, 0x41, 0x53, 0x54, 0x45,
                        0x52,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
