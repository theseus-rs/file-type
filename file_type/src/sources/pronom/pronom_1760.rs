use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1760: FileType = FileType {
    file_format: &FileFormat {
        id: 1_760,
        source_type: SourceType::Pronom,
        name: "Downloadable Sounds Audio",
        extensions: &["dls"],
        media_types: &["audio/dls"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x52, 0x49, 0x46, 0x46]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x44, 0x4C, 0x53, 0x20]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 2_741,
        }],
    },
};
