use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_883: FileFormat = FileFormat {
    id: 1_687,
    puid: "fmt/883",
    name: "Siegfried Signature File",
    extensions: &["sig"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x73, 0x66, 0x00, 0xFF])],
            },
        }],
    }],
    related_formats: &[],
};
