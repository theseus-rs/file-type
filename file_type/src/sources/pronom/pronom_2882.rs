use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2882: FileType = FileType {
    file_format: &FileFormat {
        id: 2_882,
        source_type: SourceType::Pronom,
        name: "QuarkXPress Project",
        extensions: &["qxp", "qwd", "qpt"],
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
                relationship_type: RelationshipType::HasPriorityOver,
                id: 255,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 2_883,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 2_881,
            },
        ],
    },
};
