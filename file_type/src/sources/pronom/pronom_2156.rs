use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2156: FileType = FileType {
    file_format: &FileFormat {
        id: 2_156,
        source_type: SourceType::Pronom,
        name: "RootsMagic Database",
        extensions: &["rmgc"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x53, 0x51, 0x4C, 0x69, 0x74, 0x65, 0x20, 0x66, 0x6F, 0x72, 0x6D, 0x61,
                            0x74, 0x20, 0x33, 0x00,
                        ]),
                        Token::WildcardCountRange(0, 256),
                        Token::Literal(&[
                            0x46, 0x61, 0x6D, 0x69, 0x6C, 0x79, 0x54, 0x61, 0x62, 0x6C, 0x65,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_528,
        }],
    },
};
