use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1192: FileType = FileType {
    file_format: &FileFormat {
        id: 1_192,
        source_type: SourceType::Pronom,
        name: "Audio Interchange File Format",
        extensions: &["aif", "aiff"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x46, 0x4F, 0x52, 0x4D]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x41, 0x49, 0x46, 0x46]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 221,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 687,
            },
        ],
    },
};
