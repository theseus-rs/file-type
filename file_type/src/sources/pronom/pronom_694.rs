use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_694: FileType = FileType {
    file_format: &FileFormat {
        id: 694,
        source_type: SourceType::Pronom,
        name: "AutoCAD Drawing",
        extensions: &["dwg"],
        media_types: &["image/vnd.dwg"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x43, 0x30, 0x2E, 0x30])],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 710,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 695,
            },
        ],
    },
};
