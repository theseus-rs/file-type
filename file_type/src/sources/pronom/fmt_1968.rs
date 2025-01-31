use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1968: FileFormat = FileFormat {
    id: 2_834,
    puid: "fmt/1968",
    name: "Atrac Codec File",
    extensions: &["aea"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(2_896),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAC])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xAC])],
                },
            },
        ],
    }],
    related_formats: &[],
};
