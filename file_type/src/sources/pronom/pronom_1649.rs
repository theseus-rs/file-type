use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1649: FileType = FileType {
    file_format: &FileFormat {
        id: 1_649,
        source_type: SourceType::Pronom,
        name: "ClarisWorks Database",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x02]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x42, 0x4F, 0x42, 0x4F]),
                        Token::WildcardCount(240),
                        Token::Literal(&[0x03]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_536,
        }],
    },
};
