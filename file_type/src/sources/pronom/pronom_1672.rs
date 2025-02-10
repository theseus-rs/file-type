use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1672: FileType = FileType {
    file_format: &FileFormat {
        id: 1_672,
        source_type: SourceType::Pronom,
        name: "MySQL Table Definition Format",
        extensions: &["frm"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0xFE, 0x01]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0x00]),
                        Token::WildcardCount(21),
                        Token::Literal(&[0x02]),
                        Token::WildcardCount(13),
                        Token::Literal(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 533,
        }],
    },
};
