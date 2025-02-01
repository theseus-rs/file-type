use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_744: FileFormat = FileFormat {
    id: 1_543,
    puid: "fmt/744",
    name: "ClarisWorks/AppleWorks Word Processor",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x05]),
                    Token::WildcardCount(3),
                    Token::Literal(&[0x42, 0x4F, 0x42, 0x4F]),
                    Token::WildcardCount(260),
                    Token::Literal(&[0x01]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_548,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_538,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
