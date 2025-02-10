use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1150: FileType = FileType {
    file_format: &FileFormat {
        id: 1_150,
        source_type: SourceType::Pronom,
        name: "Truevision TGA Bitmap",
        extensions: &["tga", "icb", "vda", "vst"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x54, 0x52, 0x55, 0x45, 0x56, 0x49, 0x53, 0x49, 0x4F, 0x4E, 0x2D, 0x58,
                        0x46, 0x49, 0x4C, 0x45, 0x2E, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 533,
        }],
    },
};
