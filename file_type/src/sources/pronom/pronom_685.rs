use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_685: FileType = FileType {
    file_format: &FileFormat {
        id: 685,
        source_type: SourceType::Pronom,
        name: "Microsoft Excel 2000-2003 Workbook (xls)",
        extensions: &["xlw", "xls"],
        media_types: &["application/vnd.ms-excel"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(512),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x09, 0x08]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x00, 0x06, 0x05, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 767,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 684,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 767,
            },
        ],
    },
};
