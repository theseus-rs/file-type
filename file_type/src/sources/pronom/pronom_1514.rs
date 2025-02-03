use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1514: FileFormat = FileFormat {
    id: 1_514,
    source_type: SourceType::Pronom,
    name: "Impulse Tracker Module",
    extensions: &["it"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x49, 0x4D, 0x50, 0x4D])],
            },
        }],
    }],
    related_formats: &[],
};
