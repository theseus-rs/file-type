use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_457: FileFormat = FileFormat {
    id: 457,
    source_type: SourceType::Pronom,
    name: "Aldus Freehand Drawing",
    extensions: &["fh3"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x48, 0x33, 0x31])],
            },
        }],
    }],
    related_formats: &[],
};
