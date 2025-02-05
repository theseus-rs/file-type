use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_680: FileFormat = FileFormat {
    id: 680,
    source_type: SourceType::Pronom,
    name: "Microsoft Excel 4.0 Worksheet (xls)",
    extensions: &["xls"],
    media_types: &["application/vnd.ms-excel"],
    signatures: &[Signature {
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
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 682,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSubsequentVersionOf,
            id: 679,
        },
        RelatedFormat {
            relationship_type: RelationshipType::IsSupertypeOf,
            id: 181,
        },
    ],
};
