use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1806: FileType = FileType {
    file_format: &FileFormat {
        id: 1_806,
        source_type: SourceType::Pronom,
        name: "OpenEXR",
        extensions: &["exr"],
        media_types: &["image/x-exr"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x76, 0x2F, 0x31, 0x01, 0x02])],
                },
            }],
        }],
        related_formats: &[],
    },
};
