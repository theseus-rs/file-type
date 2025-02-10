use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2691: FileType = FileType {
    file_format: &FileFormat {
        id: 2_691,
        source_type: SourceType::Pronom,
        name: "Microsoft Publisher Packaged Document",
        extensions: &["puz"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4D, 0x53, 0x43, 0x46]),
                        Token::WildcardCount(20),
                        Token::Literal(&[0x03, 0x01]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x2E]),
                        Token::Any(&[
                            &[Token::Literal(&[0x50, 0x55, 0x42])],
                            &[Token::Literal(&[0x70, 0x75, 0x62])],
                        ]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::HasPriorityOver,
            id: 801,
        }],
    },
};
