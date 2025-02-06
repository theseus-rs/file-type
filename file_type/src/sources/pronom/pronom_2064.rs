use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2064: FileFormat = FileFormat {
    id: 2_064,
    source_type: SourceType::Pronom,
    name: "SOSI",
    extensions: &["sos"],
    media_types: &["text/vnd.sosi"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x48, 0x4F, 0x44, 0x45]),
                    Token::WildcardCountRange(0, 300),
                    Token::Literal(&[0x53, 0x4F, 0x53, 0x49]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_065,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_066,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_067,
        },
        RelatedFormat {
            relationship_type: RelationshipType::HasLowerPriorityThan,
            id: 2_068,
        },
    ],
};
