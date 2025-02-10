use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};
use crate::FileType;

pub(crate) const PRONOM_1707: FileType = FileType {
    file_format: &FileFormat {
        id: 1_707,
        source_type: SourceType::Pronom,
        name: "Blender 3D",
        extensions: &["blend"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x42, 0x4C, 0x45, 0x4E, 0x44, 0x45, 0x52, 0x5F]),
                        Token::Any(&[&[Token::Literal(&[0x56])], &[Token::Literal(&[0x76])]]),
                    ],
                },
            }],
        }],
        related_formats: &[],
    },
};
