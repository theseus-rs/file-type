use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_347: FileFormat = FileFormat {
    id: 511,
    puid: "x-fmt/347",
    name: "MultiMate Text File",
    extensions: &["dox", "fnx", "pat"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(420),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x07]),
                    Token::WildcardCount(59),
                    Token::Literal(&[0x2E]),
                    Token::WildcardCount(1),
                    Token::Literal(&[0x00, 0x37]),
                    Token::WildcardCount(8),
                    Token::Literal(&[0x44, 0x24, 0x20, 0x20, 0x20, 0x20, 0x20]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
