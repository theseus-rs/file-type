use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_697: FileFormat = FileFormat {
    id: 1_496,
    puid: "fmt/697",
    name: "Additive Manufacturing File Format",
    extensions: &["amf"],
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
                    Token::WildcardCountRange(1, 256),
                    Token::Literal(&[0x3C, 0x61, 0x6D, 0x66]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 638,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
