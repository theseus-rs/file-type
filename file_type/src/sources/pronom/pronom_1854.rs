use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1854: FileFormat = FileFormat {
    id: 1_854,
    source_type: SourceType::Pronom,
    name: "QuickDraw 3D Metafile (ASCII)",
    extensions: &["3dmf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x33, 0x44, 0x4D, 0x65, 0x74, 0x61, 0x66, 0x69, 0x6C, 0x65, 0x20, 0x28,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
