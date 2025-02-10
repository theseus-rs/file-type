use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1781: FileType = FileType {
    file_format: &FileFormat {
        id: 1_781,
        source_type: SourceType::Pronom,
        name: "MagicaVoxel Vox format",
        extensions: &["vox"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x56, 0x4F, 0x58, 0x20, 0x96, 0x00, 0x00, 0x00, 0x4D, 0x41, 0x49, 0x4E,
                    ])],
                },
            }],
        }],
        related_formats: &[],
    },
};
