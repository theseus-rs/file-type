use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1094: FileFormat = FileFormat {
    id: 1_902,
    puid: "fmt/1094",
    name: "The Neuroimaging Informatics Technology Initiative File Format",
    extensions: &["nii"],
    media_types: &[],
    internal_signatures: &[
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(4),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6E, 0x69, 0x32, 0x00, 0x0D, 0x0A, 0x1A, 0x0A,
                    ])],
                },
            }],
        },
        InternalSignature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(4),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x6E, 0x2B, 0x32, 0x00, 0x0D, 0x0A, 0x1A, 0x0A,
                    ])],
                },
            }],
        },
    ],
    related_formats: &[RelatedFormat {
        id: 1_598,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
