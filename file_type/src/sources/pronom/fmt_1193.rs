use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1193: FileFormat = FileFormat {
    id: 2_003,
    puid: "fmt/1193",
    name: "ZModeler Z3D",
    extensions: &["z3d"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x5A, 0x33, 0x44, 0x4D])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_004,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
