use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_659: FileFormat = FileFormat {
    id: 659,
    source_type: SourceType::Pronom,
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
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 660,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 1_207,
        },
    ],
};
