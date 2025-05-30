use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2019: FileType = FileType {
    file_format: &FileFormat {
        id: 2_019,
        source_type: SourceType::Pronom,
        name: "COLLADA Digital Asset Exchange (DAE)",
        extensions: &["dae"],
        media_types: &["model/vnd.collada+xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                            0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22,
                        ]),
                        Token::WildcardCountRange(8, 64),
                        Token::Literal(&[0x3C, 0x43, 0x4F, 0x4C, 0x4C, 0x41, 0x44, 0x41]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 638,
        }],
    },
};
