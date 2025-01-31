use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1556: FileFormat = FileFormat {
    id: 2_381,
    puid: "fmt/1556",
    name: "Starlink Data Format",
    extensions: &["sdf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x44, 0x53])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_374,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};
