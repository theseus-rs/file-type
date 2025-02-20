use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1752: FileType = FileType {
    file_format: &FileFormat {
        id: 1_752,
        source_type: SourceType::Pronom,
        name: "Ogg FLAC Compressed Multimedia File",
        extensions: &["ogg"],
        media_types: &["audio/ogg"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4F, 0x67, 0x67, 0x53, 0x00, 0x02]),
                        Token::WildcardCount(23),
                        Token::Literal(&[0x46, 0x4C, 0x41, 0x43]),
                        Token::WildcardCount(46),
                        Token::Literal(&[0x4F, 0x67, 0x67, 0x53]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::CanBeContainedBy,
                id: 1_749,
            },
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_749,
            },
        ],
    },
};
