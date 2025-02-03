use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_143: FileFormat = FileFormat {
    id: 143,
    source_type: SourceType::Pronom,
    name: "Inkwriter/Notetaker Document",
    extensions: &["pwi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7B, 0x5C, 0x70, 0x77, 0x69])],
            },
        }],
    }],
    related_formats: &[],
};
