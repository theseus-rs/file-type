use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1769: FileType = FileType {
    file_format: &FileFormat {
        id: 1_769,
        source_type: SourceType::Pronom,
        name: "Final Draft Document",
        extensions: &["fdr"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xFD, 0xFD, 0xFD, 0xFD]),
                        Token::WildcardCountRange(12, 64),
                        Token::Literal(&[
                            0x46, 0x69, 0x6E, 0x61, 0x6C, 0x20, 0x44, 0x72, 0x61, 0x66, 0x74, 0x2C,
                            0x20, 0x49, 0x6E, 0x63,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_697,
        }],
    },
};
