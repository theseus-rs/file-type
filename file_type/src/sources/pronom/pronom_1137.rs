use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_1137: FileType = FileType {
    file_format: &FileFormat {
        id: 1_137,
        source_type: SourceType::Pronom,
        name: "Log ASCII Standard Format",
        extensions: &["las"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x7E, 0x56]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x56, 0x45, 0x52, 0x53, 0x2E]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x31, 0x2E, 0x32]),
                        Token::WildcardCountRange(0, 1),
                        Token::Literal(&[0x3A]),
                        Token::WildcardCountRange(0, 5),
                        Token::Literal(&[0x43, 0x57, 0x4C, 0x53, 0x20]),
                        Token::Any(&[
                            &[Token::Literal(&[
                                0x4C, 0x4F, 0x47, 0x20, 0x41, 0x53, 0x43, 0x49, 0x49, 0x20, 0x53,
                                0x54, 0x41, 0x4E, 0x44, 0x41, 0x52, 0x44,
                            ])],
                            &[Token::Literal(&[
                                0x4C, 0x6F, 0x67, 0x20, 0x41, 0x53, 0x43, 0x49, 0x49, 0x20, 0x53,
                                0x74, 0x61, 0x6E, 0x64, 0x61, 0x72, 0x64,
                            ])],
                            &[Token::Literal(&[
                                0x6C, 0x6F, 0x67, 0x20, 0x41, 0x53, 0x43, 0x49, 0x49, 0x20, 0x53,
                                0x74, 0x61, 0x6E, 0x64, 0x61, 0x72, 0x64,
                            ])],
                            &[Token::Literal(&[0x4C, 0x41, 0x53])],
                        ]),
                        Token::WildcardCountRange(1, 3),
                        Token::Literal(&[0x56]),
                        Token::Any(&[
                            &[Token::Literal(&[0x45, 0x52, 0x53, 0x49, 0x4F, 0x4E])],
                            &[Token::Literal(&[0x65, 0x72, 0x73, 0x69, 0x6F, 0x6E])],
                        ]),
                        Token::Literal(&[0x20, 0x31, 0x2E, 0x32]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x7E, 0x57]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x7E, 0x43]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x7E, 0x41]),
                    ],
                },
            }],
        }],
        related_formats: &[RelatedFormat {
            relationship_type: RelationshipType::IsPreviousVersionOf,
            id: 1_138,
        }],
    },
};
