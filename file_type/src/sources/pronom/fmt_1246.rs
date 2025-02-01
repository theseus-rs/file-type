use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1246: FileFormat = FileFormat {
    id: 2_064,
    puid: "fmt/1246",
    name: "SOSI",
    extensions: &["sos"],
    media_types: &["text/vnd.sosi"],
    internal_signatures: &[InternalSignature {
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
            id: 2_065,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_066,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_067,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_068,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
    ],
};
