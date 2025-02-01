use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1806: FileFormat = FileFormat {
    id: 2_657,
    puid: "fmt/1806",
    name: "Microsoft Access Database File",
    extensions: &["mdb", "mda"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x01, 0x00, 0x00, 0x00]),
                    Token::WildcardCount(1_026),
                    Token::Literal(&[0x52, 0x69, 0x63, 0x68, 0x08]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 107,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_656,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 2_659,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
    ],
};
