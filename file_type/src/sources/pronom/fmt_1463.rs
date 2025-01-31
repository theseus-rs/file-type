use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1463: FileFormat = FileFormat {
    id: 2_286,
    puid: "fmt/1463",
    name: "Ableton Live Set",
    extensions: &["als"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x3F, 0x78, 0x6D, 0x6C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6F,
                        0x6E, 0x3D, 0x22, 0x31, 0x2E, 0x30, 0x22, 0x20, 0x65, 0x6E, 0x63, 0x6F,
                        0x64, 0x69, 0x6E, 0x67, 0x3D, 0x22, 0x55, 0x54, 0x46, 0x2D, 0x38, 0x22,
                        0x3F, 0x3E, 0x0A, 0x3C, 0x41, 0x62, 0x6C, 0x65, 0x74, 0x6F, 0x6E, 0x20,
                        0x4D, 0x61, 0x6A, 0x6F, 0x72, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E,
                    ])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x3C, 0x2F, 0x4C, 0x69, 0x76, 0x65, 0x53, 0x65, 0x74, 0x3E, 0x0A, 0x3C,
                        0x2F, 0x41, 0x62, 0x6C, 0x65, 0x74, 0x6F, 0x6E, 0x3E,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[
        RelatedFormat {
            id: 386,
            relationship_type: RelationshipType::CanBeContainedBy,
        },
        RelatedFormat {
            id: 638,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
