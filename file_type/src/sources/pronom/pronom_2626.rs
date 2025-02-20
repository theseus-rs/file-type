use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2626: FileType = FileType {
    file_format: &FileFormat {
        id: 2_626,
        source_type: SourceType::Pronom,
        name: "Extensible Markup Language",
        extensions: &["xml"],
        media_types: &["application/xml", "text/xml"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x3C, 0x3F, 0x78, 0x6D, 0x6C]),
                        Token::Any(&[
                            &[Token::Literal(&[0x20])],
                            &[Token::Literal(&[0x09])],
                            &[Token::Literal(&[0x0A])],
                            &[Token::Literal(&[0x0D])],
                        ]),
                        Token::WildcardCountRange(0, 1),
                        Token::Literal(&[0x76, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E]),
                        Token::WildcardCountRange(0, 1),
                        Token::Literal(&[0x3D]),
                        Token::WildcardCountRange(0, 1),
                        Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                        Token::Literal(&[0x31, 0x2E, 0x31]),
                        Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsSubtypeOf,
                id: 638,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSupertypeOf,
                id: 931,
            },
        ],
    },
};
