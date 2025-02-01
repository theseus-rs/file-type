use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_474: FileFormat = FileFormat {
    id: 1_261,
    puid: "fmt/474",
    name: "Windows Help File",
    extensions: &["hlp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x3F, 0x5F, 0x03, 0x00]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0xFF, 0xFF, 0xFF, 0xFF]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
