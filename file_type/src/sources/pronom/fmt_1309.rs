use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1309: FileFormat = FileFormat {
    id: 2_127,
    puid: "fmt/1309",
    name: "LocoScript Professional",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x44, 0x4F, 0x43, 0x01]),
                    Token::Any(&[&[Token::Literal(&[0x02])], &[Token::Literal(&[0x03])]]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
