use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_314: FileType = FileType {
    file_format: &FileFormat {
        id: 314,
        source_type: SourceType::Pronom,
        name: "CD Audio",
        extensions: &["cda"],
        media_types: &["application/x-cdf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x52, 0x49, 0x46, 0x46, 0x24, 0x00, 0x00, 0x00, 0x43, 0x44, 0x44, 0x41,
                        0x66, 0x6D, 0x74, 0x20, 0x18,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 2_741,
        }],
    },
};
