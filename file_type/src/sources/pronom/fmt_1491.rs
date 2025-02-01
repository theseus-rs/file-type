use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1491: FileFormat = FileFormat {
    id: 2_314,
    puid: "fmt/1491",
    name: "Harvard Graphics Presentation",
    extensions: &["prs"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xDC, 0xFE]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x01, 0x00, 0x01, 0x00, 0x00]),
                    Token::WildcardCount(1),
                    Token::Literal(&[0x00, 0xC8]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
