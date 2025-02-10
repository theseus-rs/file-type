use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2326: FileType = FileType {
    file_format: &FileFormat {
        id: 2_326,
        source_type: SourceType::Pronom,
        name: "Agisoft Project File",
        extensions: &["psx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                            0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x20,
                        ]),
                        Token::WildcardCountRange(0, 4_096),
                        Token::Literal(&[
                            0x70, 0x61, 0x74, 0x68, 0x3D, 0x22, 0x7B, 0x70, 0x72, 0x6F, 0x6A, 0x65,
                            0x63, 0x74, 0x6E, 0x61, 0x6D, 0x65, 0x7D, 0x2E, 0x66, 0x69, 0x6C, 0x65,
                            0x73, 0x2F, 0x70, 0x72, 0x6F, 0x6A, 0x65, 0x63, 0x74, 0x2E, 0x7A, 0x69,
                            0x70, 0x22, 0x2F, 0x3E, 0x0A,
                        ]),
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
