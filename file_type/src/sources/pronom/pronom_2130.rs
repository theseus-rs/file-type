use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2130: FileType = FileType {
    file_format: &FileFormat {
        id: 2_130,
        source_type: SourceType::Pronom,
        name: "CorelCHART Document",
        extensions: &["cch"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x49, 0x49, 0x2A, 0x00]),
                        Token::WildcardCount(4),
                        Token::Literal(&[
                            0x33, 0x44, 0x46, 0x2E, 0x30, 0x30, 0x30, 0x32, 0x20, 0x30, 0x34, 0x64,
                            0x65, 0x63, 0x39, 0x31,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_099,
        }],
    },
};
