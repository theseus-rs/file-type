use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1915: FileFormat = FileFormat {
    id: 2_773,
    puid: "fmt/1915",
    name: "ActiveMime Object",
    extensions: &["mso"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x41, 0x63, 0x74, 0x69, 0x76, 0x65, 0x4D, 0x69, 0x6D, 0x65]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x01, 0x0F]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
