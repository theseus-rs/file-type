use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1396: FileFormat = FileFormat {
    id: 2_214,
    puid: "fmt/1396",
    name: "FinePrint",
    extensions: &["fp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x46, 0x49, 0x4E]),
                    Token::Any(&[&[Token::Literal(&[0x43])], &[Token::Literal(&[0x45])]]),
                    Token::Any(&[&[Token::Literal(&[0x01])], &[Token::Literal(&[0x02])]]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
