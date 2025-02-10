use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_406: FileType = FileType {
    file_format: &FileFormat {
        id: 406,
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
                        Token::Literal(&[0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 407,
        }],
    },
};
