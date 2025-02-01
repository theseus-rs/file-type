use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1703: FileFormat = FileFormat {
    id: 2_539,
    puid: "fmt/1703",
    name: "Persuasion Mac Document",
    extensions: &["pr2"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x14]),
                    Token::WildcardCount(5),
                    Token::Literal(&[0x00, 0x03]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 687,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_540,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_538,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
