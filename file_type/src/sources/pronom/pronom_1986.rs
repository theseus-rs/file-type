use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1986: FileType = FileType {
    file_format: &FileFormat {
        id: 1_986,
        source_type: SourceType::Pronom,
        name: "Nullsoft Streaming Video",
        extensions: &["nsv"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[
                            Token::Literal(&[0x4E, 0x53, 0x56, 0x66]),
                            Token::WildcardCountRange(24, 16_384),
                            Token::Literal(&[0x4E, 0x53, 0x56, 0x73]),
                        ],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x4E, 0x53, 0x56, 0x73])],
                    },
                }],
            },
        ],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 687,
        }],
    },
};
