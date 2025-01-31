use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1661: FileFormat = FileFormat {
    id: 2_488,
    puid: "fmt/1661",
    name: "Yamaha Wave Audio",
    extensions: &["s01", "u01", "f01", "w01"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4C, 0x4D, 0x38, 0x39, 0x35, 0x33, 0x00])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_498,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};
