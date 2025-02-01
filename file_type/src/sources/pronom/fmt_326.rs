use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_326: FileFormat = FileFormat {
    id: 1_071,
    puid: "fmt/326",
    name: "EndNote Connection File",
    extensions: &["enz"],
    media_types: &[
        "application/x-endnote-connect",
        "application/x-endnote-connection",
    ],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[
                        0x45, 0x4E, 0x44, 0x4E, 0x45, 0x4E, 0x5A, 0x33, 0x00, 0x00, 0x00,
                    ]),
                    Token::WildcardCount(5),
                    Token::Literal(&[0x00, 0x00, 0x00, 0x26, 0x00, 0x05]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
