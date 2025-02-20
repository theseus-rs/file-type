use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1821: FileType = FileType {
    file_format: &FileFormat {
        id: 1_821,
        source_type: SourceType::Pronom,
        name: "Statistical Analysis System Data (Unix)",
        extensions: &["sas7bdat", "sd7"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                            0xC2, 0xEA, 0x81, 0x60, 0xB3, 0x14, 0x11, 0xCF, 0xBD, 0x92, 0x08, 0x00,
                            0x09, 0xC7, 0x31, 0x8C, 0x18, 0x1F, 0x10, 0x11,
                        ]),
                        Token::WildcardCount(7),
                        Token::Literal(&[0x31]),
                        Token::WildcardCount(116),
                        Token::Literal(&[0x44, 0x41, 0x54, 0x41]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_400,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_823,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_825,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 1_827,
            },
        ],
    },
};
