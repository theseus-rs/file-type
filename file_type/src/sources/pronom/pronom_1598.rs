use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1598: FileType = FileType {
    file_format: &FileFormat {
        id: 1_598,
        source_type: SourceType::Pronom,
        name: "The Neuroimaging Informatics Technology Initiative File Format",
        extensions: &["nii"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(344),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x6E, 0x69, 0x31, 0x00])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(344),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x6E, 0x2B, 0x31, 0x00])],
                    },
                }],
            },
        ],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_902,
        }],
    },
};
