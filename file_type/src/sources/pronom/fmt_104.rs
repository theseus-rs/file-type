use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_104: FileFormat = FileFormat {
    id: 646,
    puid: "fmt/104",
    name: "Macromedia Flash",
    extensions: &["swf"],
    media_types: &["application/x-shockwave-flash"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x46, 0x57, 0x53, 0x01])],
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
            id: 647,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
    ],
};
