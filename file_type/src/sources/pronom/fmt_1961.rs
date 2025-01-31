use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1961: FileFormat = FileFormat {
    id: 2_826,
    puid: "fmt/1961",
    name: "Shorten (codec)",
    extensions: &["shn"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x61, 0x6A, 0x6B, 0x67])],
            },
        }],
    }],
    related_formats: &[],
};
