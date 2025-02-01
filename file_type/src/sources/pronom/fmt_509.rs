use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_509: FileFormat = FileFormat {
    id: 1_296,
    puid: "fmt/509",
    name: "Adobe PostScript Font Metrics file",
    extensions: &["pfm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x01]),
                    Token::WildcardCount(64),
                    Token::Literal(&[0x81, 0x00, 0x0A, 0x00, 0x2C, 0x01, 0x2C, 0x01]),
                    Token::WildcardCount(43),
                    Token::Literal(&[0x1E, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
