use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_3943: FileType = FileType {
    file_format: &FileFormat {
        id: 3_943,
        source_type: SourceType::Pronom,
        name: "Macintosh File System",
        extensions: &["mfs"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(1_024),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xD2, 0xD7]),
                        Token::WildcardCount(12),
                        Token::Literal(&[0x00, 0x04]),
                        Token::WildcardCount(6),
                        Token::NotLiteral(&[]),
                        Token::Literal(&[0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_913,
        }],
    },
};
