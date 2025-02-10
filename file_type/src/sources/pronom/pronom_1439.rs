use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1439: FileType = FileType {
    file_format: &FileFormat {
        id: 1_439,
        source_type: SourceType::Pronom,
        name: "MPEG-2 Elementary Stream",
        extensions: &["mpg", "mpeg", "m2v"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x00, 0x01, 0xB3]),
                        Token::WildcardCountRange(8, 256),
                        Token::Literal(&[0x00, 0x00, 0x01, 0xB5]),
                        Token::WildcardCountRange(6, 256),
                        Token::Literal(&[0x00, 0x00, 0x01, 0xB8]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::CanBeContainedBy,
                id: 660,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_448,
            },
        ],
    },
};
