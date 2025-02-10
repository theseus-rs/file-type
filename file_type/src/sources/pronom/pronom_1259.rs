use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1259: FileType = FileType {
    file_format: &FileFormat {
        id: 1_259,
        source_type: SourceType::Pronom,
        name: "Sony Digital Voice File/Sony Memory Stick Voice File",
        extensions: &["msv", "dvf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x4D, 0x53, 0x5F, 0x56, 0x4F, 0x49, 0x43, 0x45, 0x00, 0x00, 0x00, 0x50,
                        0x01, 0x02, 0x00, 0x00, 0x53, 0x4F, 0x4E, 0x59, 0x20, 0x43, 0x4F, 0x52,
                        0x50, 0x4F, 0x52, 0x41, 0x54, 0x49, 0x4F, 0x4E,
                    ])],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_746,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_747,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_748,
            },
        ],
    },
};
