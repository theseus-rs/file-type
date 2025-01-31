use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_55: FileFormat = FileFormat {
    id: 678,
    puid: "fmt/55",
    name: "Microsoft Excel 2.x Worksheet (xls)",
    extensions: &["xls"],
    media_types: &["application/vnd.ms-excel"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x09, 0x00, 0x04, 0x00]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x10, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 679,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_341,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
        RelatedFormat {
            id: 1_343,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
    ],
};
