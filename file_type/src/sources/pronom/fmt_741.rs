use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_741: FileFormat = FileFormat {
    id: 1_540,
    puid: "fmt/741",
    name: "ClarisWorks Database",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x04]),
                    Token::WildcardCount(3),
                    Token::Literal(&[0x42, 0x4F, 0x42, 0x4F]),
                    Token::WildcardCount(248),
                    Token::Literal(&[0x03]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_545,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_536,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
