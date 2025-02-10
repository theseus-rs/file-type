use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_695: FileType = FileType {
    file_format: &FileFormat {
        id: 695,
        source_type: SourceType::Pronom,
        name: "AutoCAD Drawing",
        extensions: &["dwg"],
        media_types: &["image/vnd.dwg"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x41, 0x43, 0x31, 0x2E, 0x32])],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasLowerPriorityThan,
                id: 711,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 696,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 694,
            },
        ],
    },
};
