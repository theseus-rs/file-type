use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_57: FileFormat = FileFormat {
    id: 680,
    puid: "fmt/57",
    name: "Microsoft Excel 4.0 Worksheet (xls)",
    extensions: &["xls"],
    media_types: &["application/vnd.ms-excel"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x09, 0x04, 0x06, 0x00]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x10, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 682,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 679,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 181,
            relationship_type: RelationshipType::IsSupertypeOf,
        },
    ],
};
