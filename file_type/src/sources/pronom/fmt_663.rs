use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_663: FileFormat = FileFormat {
    id: 1_462,
    puid: "fmt/663",
    name: "Industry Foundation Classes XML",
    extensions: &["ifcXML"],
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
                    Token::WildcardCountRange(0, 32),
                    Token::Literal(&[
                        0x69, 0x73, 0x6F, 0x5F, 0x31, 0x30, 0x33, 0x30, 0x33, 0x5F, 0x32, 0x38,
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
