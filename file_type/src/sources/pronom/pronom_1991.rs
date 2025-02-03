use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1991: FileFormat = FileFormat {
    id: 1_991,
    source_type: SourceType::Pronom,
    name: "Bodypaint 3D",
    extensions: &["b3d"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x41, 0x43, 0x34, 0x44, 0x42, 0x6F, 0x64, 0x79,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
