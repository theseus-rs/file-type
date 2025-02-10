use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_681: FileType = FileType {
    file_format: &FileFormat {
        id: 681,
        source_type: SourceType::Pronom,
        name: "Microsoft Excel 4.0 Workbook (xls)",
        extensions: &["xlw"],
        media_types: &["application/vnd.ms-excel"],
        signatures: &[Signature {
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
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 682,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 679,
            },
        ],
    },
};
