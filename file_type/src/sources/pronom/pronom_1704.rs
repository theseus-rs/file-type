use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1704: FileType = FileType {
    file_format: &FileFormat {
        id: 1_704,
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
                        Token::WildcardCountRange(126, 128_500),
                        Token::Literal(&[0x50, 0x45, 0x00, 0x00]),
                        Token::WildcardCount(20),
                        Token::Literal(&[0x0B, 0x01]),
                        Token::WildcardCount(66),
                        Token::Range(&[0x00, 0x00], &[0x10, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 2_645,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 774,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 775,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 776,
            },
        ],
    },
};
