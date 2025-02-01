use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_2007: FileFormat = FileFormat {
    id: 2_882,
    puid: "fmt/2007",
    name: "QuarkXPress Project",
    extensions: &["qxp", "qwd", "qpt"],
    media_types: &["application/vnd.Quark.QuarkXPress"],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00, 0x4D, 0x4D, 0x58, 0x50, 0x52])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00, 0x49, 0x49, 0x58, 0x50, 0x52])],
                },
            }],
        },
    ],
    related_formats: &[
        RelatedFormat {
            id: 255,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_883,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_881,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
