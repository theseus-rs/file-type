use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1704: FileFormat = FileFormat {
    id: 2_540,
    puid: "fmt/1704",
    name: "Persuasion Mac Document",
    extensions: &["pr3"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x29]),
                    Token::WildcardCount(5),
                    Token::Literal(&[0x56, 0x03]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_541,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_539,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
