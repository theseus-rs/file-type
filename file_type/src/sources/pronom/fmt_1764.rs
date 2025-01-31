use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1764: FileFormat = FileFormat {
    id: 2_614,
    puid: "fmt/1764",
    name: "Sony SLV File",
    extensions: &["slv"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x73, 0x6C, 0x76, 0x67]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x66, 0x69, 0x6C, 0x6C]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
