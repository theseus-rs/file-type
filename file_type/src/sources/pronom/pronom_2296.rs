use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2296: FileFormat = FileFormat {
    id: 2_296,
    source_type: SourceType::Pronom,
    name: "Archimedes Tracker Module",
    extensions: &["musx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x55, 0x53, 0x58])],
            },
        }],
    }],
    related_formats: &[],
};
