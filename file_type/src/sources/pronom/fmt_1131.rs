use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1131: FileFormat = FileFormat {
    id: 1_941,
    puid: "fmt/1131",
    name: "Gatan Digital Micrograph File Format (DM3)",
    extensions: &["dm3"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x00, 0x00, 0x00, 0x03]),
                        Token::WildcardCount(4),
                        Token::Literal(&[0x00, 0x00, 0x00]),
                        Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x01])]]),
                        Token::WildcardCount(6),
                        Token::Any(&[&[Token::Literal(&[0x14])], &[Token::Literal(&[0x15])]]),
                        Token::WildcardCountRange(2, 258),
                        Token::Literal(&[0x25, 0x25, 0x25, 0x25]),
                    ],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    ])],
                },
            },
        ],
    }],
    related_formats: &[],
};
