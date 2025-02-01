use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_758: FileFormat = FileFormat {
    id: 1_557,
    puid: "fmt/758",
    name: "Adobe Flash",
    extensions: &["swf"],
    media_types: &["application/x-shockwave-flash"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[
                ByteSequence {
                    position_type: PositionType::BOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x46, 0x57, 0x53, 0x0C])],
                    },
                },
                ByteSequence {
                    position_type: PositionType::EOF,
                    offset: Some(0),
                    regex: Regex {
                        tokens: &[Token::Literal(&[0x00, 0x00])],
                    },
                },
            ],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x43, 0x57, 0x53, 0x0C])],
                },
            }],
        },
    ],
    related_formats: &[
        RelatedFormat {
            id: 1_558,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_556,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
