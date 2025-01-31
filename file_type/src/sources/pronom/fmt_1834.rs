use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1834: FileFormat = FileFormat {
    id: 2_686,
    puid: "fmt/1834",
    name: "GoDot 4Bit Graphics Format",
    extensions: &["4bt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x47, 0x4F, 0x44, 0x30])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 687,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
