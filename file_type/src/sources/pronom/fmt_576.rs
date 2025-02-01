use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_576: FileFormat = FileFormat {
    id: 1_364,
    puid: "fmt/576",
    name: "GraphPad Prism",
    extensions: &["pzf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x43, 0x46, 0x46, 0x47, 0x52, 0x41, 0x34,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
