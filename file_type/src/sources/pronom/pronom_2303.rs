use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2303: FileType = FileType {
    file_format: &FileFormat {
        id: 2_303,
        source_type: SourceType::Pronom,
        name: "XIFF (Xerox Image File Format)",
        extensions: &["xif"],
        media_types: &["image/vnd.xiff"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x49, 0x49, 0x2A, 0x00]),
                        Token::WildcardCount(4),
                        Token::Literal(&[
                            0x20, 0x65, 0x58, 0x74, 0x65, 0x6E, 0x64, 0x65, 0x64, 0x20, 0x03,
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
