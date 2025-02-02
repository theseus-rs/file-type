use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2096: FileFormat = FileFormat {
    id: 2_096,
    source_type: SourceType::Pronom,
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
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_097,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 2_095,
        },
    ],
};
