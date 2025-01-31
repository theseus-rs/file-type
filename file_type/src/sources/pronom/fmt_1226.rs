use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1226: FileFormat = FileFormat {
    id: 2_036,
    puid: "fmt/1226",
    name: "Sparky",
    extensions: &["ucsf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x55, 0x43, 0x53, 0x46, 0x20, 0x4E, 0x4D, 0x52, 0x00]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x02]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
