use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2523: FileType = FileType {
    file_format: &FileFormat {
        id: 2_523,
        source_type: SourceType::Pronom,
        name: "PageMaker Mac Document",
        extensions: &[],
        media_types: &["application/vnd.pagemaker"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(6),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x99, 0xFF]),
                        Token::WildcardCount(100),
                        Token::Literal(&[0x50, 0x4D, 0x05, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::EquivalentTo,
                id: 246,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 2_555,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 2_522,
            },
        ],
    },
};
