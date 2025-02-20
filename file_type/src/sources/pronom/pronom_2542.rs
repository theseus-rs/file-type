use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2542: FileType = FileType {
    file_format: &FileFormat {
        id: 2_542,
        source_type: SourceType::Pronom,
        name: "Persuasion Windows Document",
        extensions: &["pr2", "at2"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x99]),
                        Token::WildcardCount(100),
                        Token::Literal(&[0x53, 0x50, 0x05, 0x00, 0x02]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_543,
        }],
    },
};
