use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1738: FileType = FileType {
    file_format: &FileFormat {
        id: 1_738,
        source_type: SourceType::Pronom,
        name: "Simple Vector Format",
        extensions: &["svf"],
        media_types: &["image/vnd-svf"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x53, 0x56, 0x46, 0x20, 0x76, 0x31])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_739,
        }],
    },
};
