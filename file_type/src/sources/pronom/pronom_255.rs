use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_255: FileType = FileType {
    file_format: &FileFormat {
        id: 255,
        source_type: SourceType::Pronom,
        name: "QuarkXPress Data File",
        extensions: &[
            "qxd", "qxt", "qxp", "qcd", "qxl", "qxb", "qwd", "qwt", "qpt",
        ],
        media_types: &["application/vnd.Quark.QuarkXPress"],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x00, 0x00, 0x4D, 0x4D, 0x58, 0x50, 0x52])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x00, 0x00, 0x49, 0x49, 0x58, 0x50, 0x52])],
                    },
                }],
            },
        ],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_450,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_451,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_484,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_135,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_136,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_137,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_138,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_139,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_140,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_141,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_142,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_143,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_144,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_145,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_146,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_262,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_263,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_264,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_317,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_318,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_881,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_882,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_883,
            },
        ],
    },
};
