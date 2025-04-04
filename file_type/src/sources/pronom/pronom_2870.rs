use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2870: FileType = FileType {
    file_format: &FileFormat {
        id: 2_870,
        source_type: SourceType::Pronom,
        name: "SPIR-V",
        extensions: &["spirv"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x03, 0x02, 0x23, 0x07, 0x00, 0x00, 0x01, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 1_260,
        }],
    },
};
