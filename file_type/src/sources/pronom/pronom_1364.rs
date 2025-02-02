use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1364: FileFormat = FileFormat {
    id: 1_364,
    source_type: SourceType::Pronom,
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
