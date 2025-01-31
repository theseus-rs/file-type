use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_66: FileFormat = FileFormat {
    id: 107,
    puid: "x-fmt/66",
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
                    Token::Literal(&[0x52, 0x69, 0x63, 0x68, 0x09]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 350,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_657,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 2_660,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
    ],
};
