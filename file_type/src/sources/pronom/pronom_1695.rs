use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1695: FileType = FileType {
    file_format: &FileFormat {
        id: 1_695,
        source_type: SourceType::Pronom,
        name: "AbiWord Document Template",
        extensions: &["awt"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x20, 0x65, 0x6E, 0x63, 0x6F,
                        0x64, 0x69, 0x6E, 0x67, 0x3D, 0x22, 0x55, 0x54, 0x46, 0x2D, 0x38, 0x22,
                        0x3F, 0x3E, 0x0A, 0x3C, 0x21, 0x44, 0x4F, 0x43, 0x54, 0x59, 0x50, 0x45,
                        0x20, 0x61, 0x62, 0x69, 0x77, 0x6F, 0x72, 0x64, 0x20, 0x50, 0x55, 0x42,
                        0x4C, 0x49, 0x43, 0x20, 0x22, 0x2D, 0x2F, 0x2F, 0x41, 0x42, 0x49, 0x53,
                        0x4F, 0x55, 0x52, 0x43, 0x45, 0x2F, 0x2F, 0x44, 0x54, 0x44, 0x20, 0x41,
                        0x57, 0x4D, 0x4C, 0x20, 0x31, 0x2E, 0x30, 0x20, 0x53, 0x74, 0x72, 0x69,
                        0x63, 0x74, 0x2F, 0x2F, 0x45, 0x4E, 0x22, 0x20, 0x22, 0x68, 0x74, 0x74,
                        0x70, 0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E, 0x61, 0x62, 0x69, 0x73,
                        0x6F, 0x75, 0x72, 0x63, 0x65, 0x2E, 0x63, 0x6F, 0x6D, 0x2F, 0x61, 0x77,
                        0x6D, 0x6C, 0x2E, 0x64, 0x74, 0x64, 0x22, 0x3E, 0x0A, 0x3C, 0x61, 0x62,
                        0x69, 0x77, 0x6F, 0x72, 0x64, 0x20, 0x74, 0x65, 0x6D, 0x70, 0x6C, 0x61,
                        0x74, 0x65, 0x3D, 0x22, 0x74, 0x72, 0x75, 0x65, 0x22,
                    ])],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 638,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 638,
            },
        ],
    },
};
