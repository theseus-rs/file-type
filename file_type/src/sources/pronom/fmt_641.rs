use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_641: FileFormat = FileFormat {
    id: 1_440,
    puid: "fmt/641",
    name: "Epson Raw Image Format",
    extensions: &["erf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4D, 0x4D, 0x00, 0x2A]),
                    Token::WildcardCount(304),
                    Token::Literal(&[
                        0x45, 0x50, 0x53, 0x4F, 0x4E, 0x20, 0x44, 0x53, 0x43, 0x20, 0x50, 0x69,
                        0x63, 0x74, 0x75, 0x72, 0x65, 0x00, 0x53, 0x45, 0x49, 0x4B, 0x4F, 0x20,
                        0x45, 0x50, 0x53, 0x4F, 0x4E, 0x20, 0x43, 0x4F, 0x52, 0x50, 0x2E, 0x00,
                        0x52, 0x2D, 0x44, 0x31,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 797,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_099,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
