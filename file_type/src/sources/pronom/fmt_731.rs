use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_731: FileFormat = FileFormat {
    id: 1_530,
    puid: "fmt/731",
    name: "Bink Video Format",
    extensions: &["bik"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x42, 0x49, 0x4B]),
                    Token::Any(&[
                        &[Token::Literal(&[0x62])],
                        &[Token::Literal(&[0x64])],
                        &[Token::Literal(&[0x66])],
                        &[Token::Literal(&[0x67])],
                        &[Token::Literal(&[0x68])],
                        &[Token::Literal(&[0x69])],
                    ]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_531,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
