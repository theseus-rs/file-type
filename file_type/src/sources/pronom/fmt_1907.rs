use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1907: FileFormat = FileFormat {
    id: 2_763,
    puid: "fmt/1907",
    name: "Micrografx Icon File",
    extensions: &["icn"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x5A, 0x5A, 0x5A, 0x5A]),
                    Token::WildcardCount(37),
                    Token::Literal(&[0x76, 0x69, 0x65, 0x77]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
