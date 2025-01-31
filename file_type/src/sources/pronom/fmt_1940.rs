use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1940: FileFormat = FileFormat {
    id: 2_801,
    puid: "fmt/1940",
    name: "EBU Subtitling Data Exchange Format",
    extensions: &["stl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(3),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x53, 0x54, 0x4C]),
                    Token::Any(&[
                        &[Token::Literal(&[0x32, 0x34])],
                        &[Token::Literal(&[0x32, 0x35])],
                        &[Token::Literal(&[0x33, 0x30])],
                    ]),
                    Token::Literal(&[0x2E, 0x30]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
