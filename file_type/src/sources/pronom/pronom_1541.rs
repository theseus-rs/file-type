use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1541: FileType = FileType {
    file_format: &FileFormat {
        id: 1_541,
        source_type: SourceType::Pronom,
        name: "ClarisWorks Painting",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x04]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x42, 0x4F, 0x42, 0x4F]),
                        Token::WildcardCount(248),
                        Token::Literal(&[0x04]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_546,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 1_536,
            },
        ],
    },
};
