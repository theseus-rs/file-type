use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1543: FileType = FileType {
    file_format: &FileFormat {
        id: 1_543,
        source_type: SourceType::Pronom,
        name: "ClarisWorks/AppleWorks Word Processor",
        extensions: &["cwk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x05]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x42, 0x4F, 0x42, 0x4F]),
                        Token::WildcardCount(260),
                        Token::Literal(&[0x01]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_548,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 1_538,
            },
        ],
    },
};
