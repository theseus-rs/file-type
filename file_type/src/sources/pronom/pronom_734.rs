use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_734: FileType = FileType {
    file_format: &FileFormat {
        id: 734,
        source_type: SourceType::Pronom,
        name: "Microsoft Word for Windows Document",
        extensions: &["doc"],
        media_types: &["application/msword"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xDB, 0xA5]),
                        Token::WildcardCount(16),
                        Token::Literal(&[0x00, 0x00, 0x00, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 688,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 733,
            },
        ],
    },
};
