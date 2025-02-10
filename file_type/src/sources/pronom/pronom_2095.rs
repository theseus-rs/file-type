use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2095: FileType = FileType {
    file_format: &FileFormat {
        id: 2_095,
        source_type: SourceType::Pronom,
        name: "Cindex Document",
        extensions: &["cdx", "tpl"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x18, 0x24, 0x00, 0x00, 0xCB, 0x00, 0x00, 0x00,
                    ])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_096,
        }],
    },
};
