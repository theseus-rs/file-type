use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_120: FileFormat = FileFormat {
    id: 769,
    puid: "fmt/120",
    name: "DROID File Collection File Format",
    extensions: &["xml"],
    media_types: &["text/xml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::WildcardCountRange(0, 50),
                    Token::Literal(&[
                        0x3C, 0x46, 0x69, 0x6C, 0x65, 0x43, 0x6F, 0x6C, 0x6C, 0x65, 0x63, 0x74,
                        0x69, 0x6F, 0x6E, 0x20,
                    ]),
                    Token::WildcardCountRange(0, 100),
                    Token::Literal(&[
                        0x3C, 0x44, 0x52, 0x4F, 0x49, 0x44, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x3E,
                    ]),
                    Token::WildcardCountRange(1, 10),
                    Token::Literal(&[
                        0x3C, 0x2F, 0x44, 0x52, 0x4F, 0x49, 0x44, 0x56, 0x65, 0x72, 0x73, 0x69,
                        0x6F, 0x6E, 0x3E,
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
