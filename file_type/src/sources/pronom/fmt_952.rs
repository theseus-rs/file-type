use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_952: FileFormat = FileFormat {
    id: 1_757,
    puid: "fmt/952",
    name: "True Audio",
    extensions: &["tta"],
    media_types: &["audio/tta"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x54, 0x41, 0x31])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_758,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
