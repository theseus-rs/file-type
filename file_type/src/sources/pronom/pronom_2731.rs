use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2731: FileFormat = FileFormat {
    id: 2_731,
    source_type: SourceType::Pronom,
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
            relationship_type: RelationshipType::HasPriorityOver,
            id: 687,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 2_732,
        },
    ],
};
