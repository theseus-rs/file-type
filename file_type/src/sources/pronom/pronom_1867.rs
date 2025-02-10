use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1867: FileType = FileType {
    file_format: &FileFormat {
        id: 1_867,
        source_type: SourceType::Pronom,
        name: "Phase One IIQ Raw Image",
        extensions: &["iiq"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x49, 0x49, 0x2A, 0x00]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x49, 0x49, 0x49, 0x49, 0x43, 0x77, 0x61, 0x52]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 672,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 673,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 752,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_099,
            },
        ],
    },
};
