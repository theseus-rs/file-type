use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1780: FileFormat = FileFormat {
    id: 1_780,
    source_type: SourceType::Pronom,
    name: "Jamcracker Tracker Module",
    extensions: &["jam"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x42, 0x65, 0x45, 0x70])],
            },
        }],
    }],
    related_formats: &[],
};
