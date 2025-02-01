use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_385: FileFormat = FileFormat {
    id: 659,
    puid: "x-fmt/385",
    name: "MPEG-1 Program Stream",
    extensions: &["mpeg", "mpg"],
    media_types: &["video/mpeg"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x00, 0x01, 0xBA]),
                    Token::WildcardCountRange(8, 12),
                    Token::Literal(&[0x00, 0x00, 0x01, 0xBB]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 660,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_207,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
    ],
};
