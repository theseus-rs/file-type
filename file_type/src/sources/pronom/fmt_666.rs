use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_666: FileFormat = FileFormat {
    id: 1_465,
    puid: "fmt/666",
    name: "ART image format",
    extensions: &["art"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4A, 0x47]),
                    Token::Any(&[&[Token::Literal(&[0x03])], &[Token::Literal(&[0x04])]]),
                    Token::Literal(&[0x0E]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
