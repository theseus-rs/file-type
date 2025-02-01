use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1414: FileFormat = FileFormat {
    id: 2_232,
    puid: "fmt/1414",
    name: "PFS:Write Document",
    extensions: &["pfs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x42, 0x00]),
                        Token::Any(&[&[Token::Literal(&[0x06])], &[Token::Literal(&[0x0C])]]),
                        Token::Literal(&[0x00]),
                        Token::WildcardCount(6),
                        Token::Literal(&[0x0A, 0x00, 0x46, 0x00]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x0E])],
                },
            },
        ],
    }],
    related_formats: &[],
};
