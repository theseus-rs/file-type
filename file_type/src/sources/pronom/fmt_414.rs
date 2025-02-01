use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_414: FileFormat = FileFormat {
    id: 1_192,
    puid: "fmt/414",
    name: "Audio Interchange File Format",
    extensions: &["aif", "aiff"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x46, 0x4F, 0x52, 0x4D]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x41, 0x49, 0x46, 0x46]),
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
            id: 687,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
