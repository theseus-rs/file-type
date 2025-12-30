use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1939: FileType = FileType {
    file_format: &FileFormat {
        id: 1_939,
        source_type: SourceType::Pronom,
        name: "PDF 2.0 - Portable Document Format",
        extensions: &["pdf"],
        media_types: &["application/pdf"],
        signatures: &[Signature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x25, 0x50, 0x44, 0x46, 0x2D, 0x32, 0x2E, 0x30,
                        ])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x25, 0x25, 0x45, 0x4F, 0x46])],
                    },
                },
            ],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_766,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_767,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_770,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 3_926,
            },
        ],
    },
};
