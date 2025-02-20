use crate::FileType;
use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_408: FileType = FileType {
    file_format: &FileFormat {
        id: 408,
        source_type: SourceType::Pronom,
        name: "Microsoft Word for MS-DOS Document",
        extensions: &["doc"],
        media_types: &["application/msword"],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[
                            0x31, 0xBE, 0x00, 0x00, 0x00, 0xAB, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                            0x00, 0x00,
                        ]),
                        Token::WildcardCount(82),
                        Token::Literal(&[0x00, 0x00]),
                        Token::WildcardCount(18),
                        Token::Any(&[&[Token::Literal(&[0x06])], &[Token::Literal(&[0x07])]]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::IsPreviousVersionOf,
                id: 2_524,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 407,
            },
        ],
    },
};
