use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1751: FileType = FileType {
    file_format: &FileFormat {
        id: 1_751,
        source_type: SourceType::Pronom,
        name: "Ogg Opus Codec Compressed Multimedia File",
        extensions: &["ogg", "opus"],
        media_types: &["audio/ogg", "audio/opus"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4F, 0x67, 0x67, 0x53, 0x00, 0x02]),
                        Token::WildcardCount(22),
                        Token::Literal(&[0x4F, 0x70, 0x75, 0x73, 0x48, 0x65, 0x61, 0x64]),
                        Token::WildcardCount(11),
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
