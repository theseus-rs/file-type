use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1095: FileFormat = FileFormat {
    id: 1_903,
    puid: "fmt/1095",
    name: "PEA Archive Format",
    extensions: &["pea"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xEA]),
                    Token::WildcardCount(9),
                    Token::Literal(&[0x00, 0x00, 0x50, 0x4F, 0x44, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
