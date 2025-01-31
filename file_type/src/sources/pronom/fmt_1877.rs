use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1877: FileFormat = FileFormat {
    id: 2_731,
    puid: "fmt/1877",
    name: "GST Art File",
    extensions: &["art"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(16),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x53, 0x54, 0x3A, 0x41, 0x52, 0x54, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 687,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_732,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
    ],
};
