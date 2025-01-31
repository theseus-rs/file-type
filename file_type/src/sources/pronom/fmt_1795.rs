use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1795: FileFormat = FileFormat {
    id: 2_645,
    puid: "fmt/1795",
    name: "Asymetrix Toolbook File",
    extensions: &["tbk", "sbk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
            id: 1_704,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 1_257,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
