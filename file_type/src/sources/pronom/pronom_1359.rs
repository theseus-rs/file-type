use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1359: FileFormat = FileFormat {
    id: 1_359,
    source_type: SourceType::Pronom,
    name: "Domino XML Document Export",
    extensions: &["dxl"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x3C, 0x64, 0x6F, 0x63, 0x75, 0x6D, 0x65, 0x6E, 0x74, 0x20, 0x66, 0x6F,
                        0x72, 0x6D, 0x3D,
                    ]),
                    Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                    Token::WildcardCountRange(0, 32),
                    Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                    Token::Literal(&[0x3E]),
                    Token::WildcardCountRange(0, 32),
                    Token::Literal(&[
                        0x3C, 0x6E, 0x6F, 0x74, 0x65, 0x69, 0x6E, 0x66, 0x6F, 0x20, 0x6E, 0x6F,
                        0x74, 0x65, 0x69, 0x64, 0x3D,
                    ]),
                    Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                    Token::WildcardCountRange(0, 32),
                    Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                    Token::WildcardCountRange(0, 32),
                    Token::Literal(&[0x75, 0x6E, 0x69, 0x64, 0x3D]),
                    Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                    Token::WildcardCount(32),
                    Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 638,
    }],
};
