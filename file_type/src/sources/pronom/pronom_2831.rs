use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2831: FileType = FileType {
    file_format: &FileFormat {
        id: 2_831,
        source_type: SourceType::Pronom,
        name: "Final Writer Document",
        extensions: &["fw"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x46, 0x4F, 0x52, 0x4D]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x53, 0x57, 0x52, 0x54]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 221,
        }],
    },
};
