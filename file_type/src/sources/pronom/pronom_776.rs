use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_776: FileType = FileType {
    file_format: &FileFormat {
        id: 776,
        source_type: SourceType::Pronom,
        name: "Windows Portable Executable",
        extensions: &["exe", "dll", "sys"],
        media_types: &["application/vnd.microsoft.portable-executable"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4D, 0x5A]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x50, 0x45, 0x00, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_704,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_705,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 774,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 775,
            },
        ],
    },
};
