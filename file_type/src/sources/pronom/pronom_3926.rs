use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_3926: FileType = FileType {
    file_format: &FileFormat {
        id: 3_926,
        source_type: SourceType::Pronom,
        name: "PDF/UA Portable Document Format",
        extensions: &["pdf"],
        media_types: &["application/pdf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x25, 0x50, 0x44, 0x46, 0x2D, 0x32, 0x2E, 0x30]),
                        Token::AnyWildcard,
                        Token::Literal(&[
                            0x70, 0x64, 0x66, 0x75, 0x61, 0x69, 0x64, 0x3A, 0x70, 0x61, 0x72, 0x74,
                            0x3D, 0x22, 0x32, 0x22,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_939,
        }],
    },
};
