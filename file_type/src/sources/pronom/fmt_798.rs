use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_798: FileFormat = FileFormat {
    id: 1_598,
    puid: "fmt/798",
    name: "The Neuroimaging Informatics Technology Initiative File Format",
    extensions: &["nii"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(344),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6E, 0x69, 0x31, 0x00])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(344),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x6E, 0x2B, 0x31, 0x00])],
                },
            }],
        },
    ],
    related_formats: &[RelatedFormat {
        id: 1_902,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
