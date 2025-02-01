use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1278: FileFormat = FileFormat {
    id: 2_096,
    puid: "fmt/1278",
    name: "Cindex Document",
    extensions: &["ucdx", "utpl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x94, 0x70, 0x00, 0x00, 0x2C,
                    0x01,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_097,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_095,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
