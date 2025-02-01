use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1788: FileFormat = FileFormat {
    id: 2_638,
    puid: "fmt/1788",
    name: "Gunpaint Image File",
    extensions: &["gun"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x40]),
                    Token::WildcardCount(1_000),
                    Token::Literal(&[0x47, 0x55, 0x4E, 0x50, 0x41, 0x49, 0x4E, 0x54]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
