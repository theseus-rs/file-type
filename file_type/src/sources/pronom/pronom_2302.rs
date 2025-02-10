use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2302: FileType = FileType {
    file_format: &FileFormat {
        id: 2_302,
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
                            0x58, 0x45, 0x52, 0x4F, 0x58, 0x20, 0x44, 0x49, 0x46, 0x46, 0x02,
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
