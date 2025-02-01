use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1775: FileFormat = FileFormat {
    id: 2_625,
    puid: "fmt/1775",
    name: "Calc602 Project File",
    extensions: &["pc6"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x53, 0x6F, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x36, 0x30, 0x32, 0x0D,
                        0x0A, 0x43, 0x61, 0x6C, 0x63, 0x36, 0x30, 0x32, 0x20, 0x76, 0x2E,
                    ]),
                    Token::WildcardCountRange(1, 4),
                    Token::Literal(&[0x20, 0x57, 0x6F, 0x72, 0x6B, 0x53, 0x70, 0x61, 0x63, 0x65]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_549,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 2_617,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
    ],
};
