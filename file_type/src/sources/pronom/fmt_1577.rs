use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1577: FileFormat = FileFormat {
    id: 2_402,
    puid: "fmt/1577",
    name: "Spectrum 512 Extended",
    extensions: &["spx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x50, 0x58, 0x01])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_403,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
