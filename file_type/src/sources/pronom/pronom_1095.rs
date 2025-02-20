use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1095: FileType = FileType {
    file_format: &FileFormat {
        id: 1_095,
        source_type: SourceType::Pronom,
        name: "Paradox Database Table",
        extensions: &["db"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(2),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x08]),
                        Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x02])]]),
                        Token::Range(&[0x01], &[0x04]),
                        Token::WildcardCount(31),
                        Token::Literal(&[0x00, 0x00, 0x00, 0x00]),
                        Token::WildcardCount(16),
                        Token::Range(&[0x03], &[0x04]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_096,
        }],
    },
};
