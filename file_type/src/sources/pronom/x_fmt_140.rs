use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_140: FileFormat = FileFormat {
    id: 201,
    puid: "x-fmt/140",
    name: "Silicon Graphics Image",
    extensions: &["bw", "rgb"],
    media_types: &["image/x-sgi-bw"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x01, 0xDA]),
                    Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x01])]]),
                    Token::Any(&[&[Token::Literal(&[0x01])], &[Token::Literal(&[0x02])]]),
                    Token::Literal(&[0x00]),
                    Token::Any(&[
                        &[Token::Literal(&[0x01])],
                        &[Token::Literal(&[0x02])],
                        &[Token::Literal(&[0x03])],
                    ]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x00]),
                    Token::Any(&[
                        &[Token::Literal(&[0x01])],
                        &[Token::Literal(&[0x03])],
                        &[Token::Literal(&[0x04])],
                    ]),
                    Token::WildcardCount(80),
                    Token::Literal(&[0x00]),
                    Token::Any(&[
                        &[Token::Literal(&[0x00])],
                        &[Token::Literal(&[0x01])],
                        &[Token::Literal(&[0x02])],
                        &[Token::Literal(&[0x03])],
                    ]),
                    Token::WildcardCount(368),
                    Token::Literal(&[
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00,
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
