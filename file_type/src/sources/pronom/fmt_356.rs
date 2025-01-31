use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_356: FileFormat = FileFormat {
    id: 1_102,
    puid: "fmt/356",
    name: "Adaptive Multi-Rate Audio",
    extensions: &["amr"],
    media_types: &["audio/amr"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x23, 0x21, 0x41, 0x4D, 0x52, 0x0A])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_103,
        relationship_type: RelationshipType::CanBeContainedBy,
    }],
};
