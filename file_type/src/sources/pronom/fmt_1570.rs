use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1570: FileFormat = FileFormat {
    id: 2_395,
    puid: "fmt/1570",
    name: "ISDOC Information System Document",
    extensions: &["isdoc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(16),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x69, 0x73, 0x64, 0x6F, 0x63, 0x2E, 0x63, 0x7A, 0x2F, 0x6E, 0x61, 0x6D, 0x65,
                    0x73, 0x70, 0x61, 0x63, 0x65, 0x2F, 0x32, 0x30, 0x31, 0x33, 0x22, 0x20, 0x76,
                    0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E, 0x3D, 0x22, 0x36, 0x2E, 0x30, 0x2E, 0x31,
                    0x22,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 638,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_392,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_392,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 2_396,
            relationship_type: RelationshipType::IsSubtypeOf,
        },
    ],
};
