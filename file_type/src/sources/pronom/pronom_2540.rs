use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2540: FileType = FileType {
    file_format: &FileFormat {
        id: 2_540,
        source_type: SourceType::Pronom,
        name: "Persuasion Mac Document",
        extensions: &["pr3"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x29]),
                        Token::WildcardCount(5),
                        Token::Literal(&[0x56, 0x03]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 2_541,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 2_539,
            },
        ],
    },
};
