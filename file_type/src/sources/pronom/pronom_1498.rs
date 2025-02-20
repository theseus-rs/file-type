use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1498: FileType = FileType {
    file_format: &FileFormat {
        id: 1_498,
        source_type: SourceType::Pronom,
        name: "Industry Foundation Classes",
        extensions: &["ifc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x49, 0x53, 0x4F, 0x2D, 0x31, 0x30, 0x33, 0x30, 0x33, 0x2D, 0x32, 0x31,
                        ]),
                        Token::WildcardCountRange(0, 4_096),
                        Token::Literal(&[
                            0x46, 0x49, 0x4C, 0x45, 0x5F, 0x53, 0x43, 0x48, 0x45, 0x4D, 0x41,
                        ]),
                        Token::WildcardCountRange(0, 1),
                        Token::Literal(&[0x28]),
                        Token::WildcardCountRange(0, 1),
                        Token::Literal(&[0x28, 0x27, 0x49, 0x46, 0x43, 0x32, 0x58, 0x33]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_497,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_499,
            },
        ],
    },
};
