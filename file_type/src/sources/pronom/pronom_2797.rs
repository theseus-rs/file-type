use crate::FileType;
use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const PRONOM_2797: FileType = FileType {
    file_format: &FileFormat {
        id: 2_797,
        source_type: SourceType::Pronom,
        name: "PCRaster",
        extensions: &["csf", "map"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x55, 0x55, 0x20, 0x43, 0x52, 0x4F, 0x53, 0x53, 0x20, 0x53, 0x59,
                        0x53, 0x54, 0x45, 0x4D, 0x20, 0x4D, 0x41, 0x50, 0x20, 0x46, 0x4F, 0x52,
                        0x4D, 0x41, 0x54,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
