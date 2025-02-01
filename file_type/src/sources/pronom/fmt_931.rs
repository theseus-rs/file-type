use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_931: FileFormat = FileFormat {
    id: 1_736,
    puid: "fmt/931",
    name: "Mathcad Document",
    extensions: &["mcd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x2E, 0x4D, 0x43, 0x41, 0x44, 0x20])],
            },
        }],
    }],
    related_formats: &[],
};
