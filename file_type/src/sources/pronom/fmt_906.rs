use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_906: FileFormat = FileFormat {
    id: 1_711,
    puid: "fmt/906",
    name: "Variant Call Format",
    extensions: &["vcf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x23, 0x66, 0x69, 0x6C, 0x65, 0x66, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x3D,
                    0x56, 0x43, 0x46, 0x76, 0x32,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_712,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_710,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
