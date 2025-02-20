use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1440: FileType = FileType {
    file_format: &FileFormat {
        id: 1_440,
        source_type: SourceType::Pronom,
        name: "Epson Raw Image Format",
        extensions: &["erf"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4D, 0x4D, 0x00, 0x2A]),
                        Token::WildcardCount(304),
                        Token::Literal(&[
                            0x45, 0x50, 0x53, 0x4F, 0x4E, 0x20, 0x44, 0x53, 0x43, 0x20, 0x50, 0x69,
                            0x63, 0x74, 0x75, 0x72, 0x65, 0x00, 0x53, 0x45, 0x49, 0x4B, 0x4F, 0x20,
                            0x45, 0x50, 0x53, 0x4F, 0x4E, 0x20, 0x43, 0x4F, 0x52, 0x50, 0x2E, 0x00,
                            0x52, 0x2D, 0x44, 0x31,
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 797,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_099,
            },
        ],
    },
};
