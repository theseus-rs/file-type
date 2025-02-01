use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1306: FileFormat = FileFormat {
    id: 2_124,
    puid: "fmt/1306",
    name: "LocoScript Document",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4A, 0x4F, 0x59, 0x01]),
                    Token::Any(&[&[Token::Literal(&[0x03])], &[Token::Literal(&[0x04])]]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
