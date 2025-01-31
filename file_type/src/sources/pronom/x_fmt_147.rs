use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_147: FileFormat = FileFormat {
    id: 208,
    puid: "x-fmt/147",
    name: "Paradox Database Table",
    extensions: &["db"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(2),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x08]),
                    Token::Any(&[&[Token::Literal(&[0x00])], &[Token::Literal(&[0x02])]]),
                    Token::Range(&[0x01], &[0x20]),
                    Token::WildcardCount(51),
                    Token::Literal(&[0x0C]),
                    Token::WildcardCount(34),
                    Token::Literal(&[0x00, 0x00, 0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
