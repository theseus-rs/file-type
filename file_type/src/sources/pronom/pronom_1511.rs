use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1511: FileType = FileType {
    file_format: &FileFormat {
        id: 1_511,
        source_type: SourceType::Pronom,
        name: "RF64",
        extensions: &["wav", "rf64"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x52, 0x46, 0x36, 0x34]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x57, 0x41, 0x56, 0x45, 0x64, 0x73, 0x36, 0x34]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_512,
        }],
    },
};
