use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1102: FileFormat = FileFormat {
    id: 1_102,
    source_type: SourceType::Pronom,
    name: "Adaptive Multi-Rate Audio",
    extensions: &["amr"],
    media_types: &["audio/amr"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x21, 0x41, 0x4D, 0x52, 0x0A])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::CanBeContainedBy,
        id: 1_103,
    }],
};
