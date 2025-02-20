use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2659: FileType = FileType {
    file_format: &FileFormat {
        id: 2_659,
        source_type: SourceType::Pronom,
        name: "Microsoft Access Encrypted Database File",
        extensions: &["mdb", "mda"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x77, 0x2C, 0x53, 0x20]),
                        Token::WildcardCount(1_030),
                        Token::Literal(&[0x69]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 2_660,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 2_658,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 2_657,
            },
        ],
    },
};
