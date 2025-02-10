use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_195: FileType = FileType {
    file_format: &FileFormat {
        id: 195,
        source_type: SourceType::Pronom,
        name: "Audio Interchange File Format (compressed)",
        extensions: &["aifc"],
        media_types: &["audio/x-aiff"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x46, 0x4F, 0x52, 0x4D]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x41, 0x49, 0x46, 0x43]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 221,
        }],
    },
};
