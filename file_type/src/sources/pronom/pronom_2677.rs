use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2677: FileType = FileType {
    file_format: &FileFormat {
        id: 2_677,
        source_type: SourceType::Pronom,
        name: "Audacity Project File",
        extensions: &["aup3"],
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
                        Token::WildcardCount(52),
                        Token::Literal(&[0x41, 0x55, 0x44, 0x59]),
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
