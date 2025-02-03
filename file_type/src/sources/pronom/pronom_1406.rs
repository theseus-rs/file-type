use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1406: FileFormat = FileFormat {
    id: 1_406,
    source_type: SourceType::Pronom,
    name: "ARJ File Format",
    extensions: &["arj"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x60, 0xEA])],
            },
        }],
    }],
    related_formats: &[],
};
