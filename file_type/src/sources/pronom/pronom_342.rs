use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_342: FileType = FileType {
    file_format: &FileFormat {
        id: 342,
        source_type: SourceType::Pronom,
        name: "WordStar for MS-DOS Document",
        extensions: &["ws"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x1D, 0x7D, 0x00, 0x00, 0x55]),
                        Token::WildcardCount(120),
                        Token::Literal(&[0x7D, 0x00, 0x1D]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 343,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 285,
            },
        ],
    },
};
