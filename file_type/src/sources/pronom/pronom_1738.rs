use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1738: FileFormat = FileFormat {
    id: 1_738,
    source_type: SourceType::Pronom,
    name: "Simple Vector Format",
    extensions: &["svf"],
    media_types: &["image/vnd-svf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x56, 0x46, 0x20, 0x76, 0x31])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 1_739,
    }],
};
