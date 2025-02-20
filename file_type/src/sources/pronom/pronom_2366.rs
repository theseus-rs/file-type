use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2366: FileType = FileType {
    file_format: &FileFormat {
        id: 2_366,
        source_type: SourceType::Pronom,
        name: "Visual Basic Form File",
        extensions: &["frm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x56, 0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E, 0x20]),
                        Token::WildcardCountRange(1, 4),
                        Token::Literal(&[
                            0x0D, 0x0A, 0x42, 0x65, 0x67, 0x69, 0x6E, 0x20, 0x56, 0x42, 0x2E, 0x46,
                            0x6F, 0x72, 0x6D, 0x20, 0x66, 0x72, 0x6D,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 2_399,
        }],
    },
};
