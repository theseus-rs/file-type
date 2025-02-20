use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1902: FileType = FileType {
    file_format: &FileFormat {
        id: 1_902,
        source_type: SourceType::Pronom,
        name: "The Neuroimaging Informatics Technology Initiative File Format",
        extensions: &["nii"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(4),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x6E, 0x69, 0x32, 0x00, 0x0D, 0x0A, 0x1A, 0x0A,
                        ])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(4),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x6E, 0x2B, 0x32, 0x00, 0x0D, 0x0A, 0x1A, 0x0A,
                        ])],
                    },
                }],
            },
        ],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 1_598,
        }],
    },
};
