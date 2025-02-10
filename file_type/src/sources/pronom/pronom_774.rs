use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_774: FileType = FileType {
    file_format: &FileFormat {
        id: 774,
        source_type: SourceType::Pronom,
        name: "MS-DOS Executable",
        extensions: &["exe"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x4D, 0x5A])],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 775,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 776,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_704,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_705,
            },
        ],
    },
};
