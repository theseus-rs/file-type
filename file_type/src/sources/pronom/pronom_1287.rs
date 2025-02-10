use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1287: FileType = FileType {
    file_format: &FileFormat {
        id: 1_287,
        source_type: SourceType::Pronom,
        name: "Internet Explorer for Mac cache file",
        extensions: &["waf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x2E, 0x57, 0x41, 0x46, 0x00, 0x00, 0x00, 0x4C,
                    ])],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 639,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 640,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 641,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 642,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 645,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_258,
            },
        ],
    },
};
