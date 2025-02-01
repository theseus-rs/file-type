use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1997: FileFormat = FileFormat {
    id: 2_871,
    puid: "fmt/1997",
    name: "IMF Package Asset Map",
    extensions: &["xml"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x3D,
                    ]),
                    Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                    Token::Literal(&[0x31, 0x2E, 0x30]),
                    Token::Any(&[&[Token::Literal(&[0x22])], &[Token::Literal(&[0x27])]]),
                    Token::WildcardCountRange(1, 1_024),
                    Token::Literal(&[0x3C, 0x41, 0x73, 0x73, 0x65, 0x74, 0x4D, 0x61, 0x70]),
                    Token::WildcardCountRange(5, 1_024),
                    Token::Literal(&[
                        0x68, 0x74, 0x74, 0x70, 0x3A, 0x2F, 0x2F, 0x77, 0x77, 0x77, 0x2E, 0x73,
                        0x6D, 0x70, 0x74, 0x65, 0x2D, 0x72, 0x61, 0x2E, 0x6F, 0x72, 0x67,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 638,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
