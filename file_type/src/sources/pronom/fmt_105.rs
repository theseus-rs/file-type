use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_105: FileFormat = FileFormat {
    id: 647,
    puid: "fmt/105",
    name: "Macromedia Flash",
    extensions: &["swf"],
    media_types: &["application/x-shockwave-flash"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x57, 0x53, 0x02])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x00, 0x00])],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            id: 687,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 648,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 646,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
