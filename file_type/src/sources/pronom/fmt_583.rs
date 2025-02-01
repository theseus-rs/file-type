use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_583: FileFormat = FileFormat {
    id: 1_371,
    puid: "fmt/583",
    name: "Vector Markup Language",
    extensions: &["vml", "html", "htm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x78, 0x6D, 0x6C, 0x6E, 0x73, 0x3A, 0x76, 0x3D, 0x22, 0x75, 0x72, 0x6E, 0x3A,
                    0x73, 0x63, 0x68, 0x65, 0x6D, 0x61, 0x73, 0x2D, 0x6D, 0x69, 0x63, 0x72, 0x6F,
                    0x73, 0x6F, 0x66, 0x74, 0x2D, 0x63, 0x6F, 0x6D, 0x3A, 0x76, 0x6D, 0x6C, 0x22,
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
            id: 645,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
