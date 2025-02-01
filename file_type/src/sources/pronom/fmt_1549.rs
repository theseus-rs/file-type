use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1549: FileFormat = FileFormat {
    id: 2_374,
    puid: "fmt/1549",
    name: "Bentley Microstation Hidden Line File",
    extensions: &["hln"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x08, 0x09, 0xFE])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(0),
                regex: Regex {
                    tokens: &[Token::Literal(&[0xFF, 0xFF])],
                },
            },
        ],
    }],
    related_formats: &[RelatedFormat {
        id: 2_176,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};
