use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_107: FileFormat = FileFormat {
    id: 107,
    source_type: SourceType::Pronom,
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
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 350,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 2_657,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 2_660,
        },
    ],
};
