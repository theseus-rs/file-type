use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1124: FileFormat = FileFormat {
    id: 1_934,
    puid: "fmt/1124",
    name: "Origin Project Format",
    extensions: &["opju", "oggu", "ogmu", "ogwu"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x43, 0x50, 0x59, 0x55, 0x41, 0x20]),
                    Token::Range(&[0x31], &[0x39]),
                    Token::Literal(&[0x2E]),
                    Token::WildcardCountRange(7, 13),
                    Token::Literal(&[0x0A]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
