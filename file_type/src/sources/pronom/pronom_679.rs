use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_679: FileType = FileType {
    file_format: &FileFormat {
        id: 679,
        source_type: SourceType::Pronom,
        name: "Microsoft Excel 3.0 Worksheet (xls)",
        extensions: &["xls"],
        media_types: &["application/vnd.ms-excel"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x09, 0x02, 0x06, 0x00]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x10, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 680,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 681,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 678,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 1_342,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 1_344,
            },
        ],
    },
};
