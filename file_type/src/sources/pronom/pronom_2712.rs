use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2712: FileType = FileType {
    file_format: &FileFormat {
        id: 2_712,
        source_type: SourceType::Pronom,
        name: "Microsoft Excel Workspace File",
        extensions: &["xlw"],
        media_types: &["application/vnd.ms-excel"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x09, 0x08, 0x08, 0x00, 0x00, 0x05, 0x00, 0x01]),
                        Token::WildcardCount(3),
                        Token::Literal(&[
                            0x07, 0x42, 0x00, 0x02, 0x00, 0xE4, 0x04, 0x38, 0x00, 0x04, 0x00,
                        ]),
                        Token::WildcardCount(1),
                        Token::Literal(&[0x00, 0x05, 0x00, 0x3D, 0x00, 0x12, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 186,
        }],
    },
};
