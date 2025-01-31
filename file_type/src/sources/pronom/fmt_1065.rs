use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1065: FileFormat = FileFormat {
    id: 1_872,
    puid: "fmt/1065",
    name: "Portable Database",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x21, 0x3C, 0x3C, 0x50, 0x44, 0x42, 0x3A, 0x49, 0x49, 0x3E, 0x3E, 0x21,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_874,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_873,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_871,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
