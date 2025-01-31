use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_58: FileFormat = FileFormat {
    id: 681,
    puid: "fmt/58",
    name: "Microsoft Excel 4.0 Workbook (xls)",
    extensions: &["xlw"],
    media_types: &["application/vnd.ms-excel"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x09, 0x04, 0x06, 0x00]),
                    Token::Any(&[
                        &[Token::Literal(&[0x00, 0x00])],
                        &[Token::Literal(&[0x00, 0x04])],
                    ]),
                    Token::Literal(&[0x00, 0x01]),
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
    ],
};
