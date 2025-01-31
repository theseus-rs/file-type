use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1909: FileFormat = FileFormat {
    id: 2_765,
    puid: "fmt/1909",
    name: "TibetDoc Word Document",
    extensions: &["dct"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(1),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x50, 0x4B, 0x54, 0x43]),
                    Token::WildcardCount(3),
                    Token::Literal(&[0x54, 0x69, 0x62, 0x65, 0x74, 0x44, 0x6F, 0x63]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
