use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1757: FileType = FileType {
    file_format: &FileFormat {
        id: 1_757,
        source_type: SourceType::Pronom,
        name: "True Audio",
        extensions: &["tta"],
        media_types: &["audio/tta"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x54, 0x54, 0x41, 0x31])],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_758,
        }],
    },
};
