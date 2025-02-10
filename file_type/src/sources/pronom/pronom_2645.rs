use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};
use crate::FileType;

pub(crate) const PRONOM_2645: FileType = FileType {
    file_format: &FileFormat {
        id: 2_645,
        source_type: SourceType::Pronom,
        name: "Asymetrix Toolbook File",
        extensions: &["tbk", "sbk"],
        media_types: &[],
        signatures: &[Signature {
            byte_sequences: &[ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x4D, 0x5A]),
                        Token::WildcardCountRange(126, 128_500),
                        Token::Literal(&[0x50, 0x45, 0x00, 0x00]),
                        Token::WildcardCount(20),
                        Token::Literal(&[0x0B, 0x01]),
                        Token::WildcardCount(66),
                        Token::Range(&[0x00, 0x00], &[0x10, 0x00]),
                        Token::AnyWildcard,
                        Token::Literal(&[0x54, 0x00, 0x6F, 0x00, 0x6F, 0x00, 0x6C, 0x00]),
                        Token::Any(&[&[Token::Literal(&[0x42])], &[Token::Literal(&[0x62])]]),
                        Token::Literal(&[0x00, 0x6F, 0x00, 0x6F, 0x00, 0x6B, 0x00]),
                    ],
                },
            }],
        }],
        related_formats: &[
            RelatedFormat {
                relationship_type: RelationshipType::HasPriorityOver,
                id: 1_704,
            },
            RelatedFormat {
                relationship_type: RelationshipType::IsSubsequentVersionOf,
                id: 1_257,
            },
        ],
    },
};
