use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2775: FileType = FileType {
    file_format: &FileFormat {
        id: 2_775,
        source_type: SourceType::Pronom,
        name: "BigTIFF",
        extensions: &["tif", "tf8", "btf"],
        media_types: &[],
        signatures: &[
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x49, 0x49, 0x2B, 0x00, 0x08, 0x00, 0x00, 0x00,
                        ])],
                    },
                }],
            },
            Signature {
                byte_sequences: &[ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[
                            0x4D, 0x4D, 0x00, 0x2B, 0x00, 0x08, 0x00, 0x00,
                        ])],
                    },
                }],
            },
        ],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_702,
        }],
    },
};
