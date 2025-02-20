use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1122: FileType = FileType {
    file_format: &FileFormat {
        id: 1_122,
        source_type: SourceType::Pronom,
        name: "FoxPro Compound Index File",
        extensions: &["cdx"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00]),
                        Token::SingleWildcard,
                        Token::Literal(&[0x00, 0x00]),
                        Token::WildcardCount(8),
                        Token::Literal(&[0x0A, 0x00]),
                        Token::NotLiteral(&[0x00]),
                        Token::WildcardCount(487),
                        Token::Any(&[
                            &[Token::Literal(&[0x00, 0x00])],
                            &[Token::Literal(&[0x00, 0x01])],
                        ]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x01, 0x00, 0x00, 0x00, 0x01, 0x00]),
                        Token::WildcardCountRange(512, 4_608),
                        Token::Literal(&[0x03]),
                        Token::WildcardCount(3),
                        Token::Literal(&[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0xFF, 0xFF]),
                        Token::WildcardCount(2),
                        Token::Literal(&[0x0F, 0x0F]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 211,
        }],
    },
};
