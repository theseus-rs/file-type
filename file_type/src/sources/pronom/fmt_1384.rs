use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1384: FileFormat = FileFormat {
    id: 2_202,
    puid: "fmt/1384",
    name: "Embedded OpenType (EOT) File Format",
    extensions: &["eot"],
    media_types: &["application/vnd.ms-fontobject"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(8),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x02, 0x00, 0x02, 0x00]),
                    Token::WildcardCount(22),
                    Token::Literal(&[0x4C, 0x50]),
                    Token::WildcardCount(36),
                    Token::Literal(&[0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
