use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1497: FileType = FileType {
    file_format: &FileFormat {
        id: 1_497,
        source_type: SourceType::Pronom,
        name: "Standard for the Exchange of Product model data",
        extensions: &["step", "stp", "p21"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x49, 0x53, 0x4F, 0x2D, 0x31, 0x30, 0x33, 0x30, 0x33, 0x2D, 0x32, 0x31,
                        0x3B,
                    ])],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_458,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_498,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_499,
            },
        ],
    },
};
