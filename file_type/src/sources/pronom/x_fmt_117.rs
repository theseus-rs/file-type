use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_117: FileFormat = FileFormat {
    id: 169,
    puid: "x-fmt/117",
    name: "Lotus 1-2-3 Worksheet",
    extensions: &["wks"],
    media_types: &["application/vnd.lotus-1-2-3", "application/x-123"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x00, 0x02, 0x00, 0x04, 0x04])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 894,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 166,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 167,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 168,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
    ],
};
