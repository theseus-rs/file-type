use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_407: FileFormat = FileFormat {
    id: 772,
    puid: "x-fmt/407",
    name: "PostScript",
    extensions: &["ps"],
    media_types: &["application/postscript"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x25, 0x21, 0x50, 0x53, 0x2D, 0x41, 0x64, 0x6F, 0x62, 0x65, 0x2D, 0x32, 0x2E,
                    0x31,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 86,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 331,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 332,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_073,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 773,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 771,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
