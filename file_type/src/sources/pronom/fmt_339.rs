use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_339: FileFormat = FileFormat {
    id: 1_084,
    puid: "fmt/339",
    name: "Interchange File Format 8-bit Sampled Voice",
    extensions: &["iff", "8svx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x46, 0x4F, 0x52, 0x4D]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x38, 0x53, 0x56, 0x58, 0x56, 0x48, 0x44, 0x52]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 221,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 221,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
