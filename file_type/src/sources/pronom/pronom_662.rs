use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_662: FileType = FileType {
    file_format: &FileFormat {
        id: 662,
        source_type: SourceType::Pronom,
        name: "Virtual Reality Modeling Language",
        extensions: &["wrl"],
        media_types: &["model/vrml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x23, 0x56, 0x52, 0x4D, 0x4C, 0x20, 0x56, 0x32, 0x2E, 0x30, 0x20, 0x75,
                        0x74, 0x66, 0x38,
                    ])],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 1_367,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 661,
            },
        ],
    },
};
