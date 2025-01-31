use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_649: FileFormat = FileFormat {
    id: 1_448,
    puid: "fmt/649",
    name: "MPEG-1 Elementary Stream",
    extensions: &["mpg", "mpeg", "m1v"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x00, 0x01, 0xB3]),
                    Token::WildcardCountRange(8, 136),
                    Token::Literal(&[0x00, 0x00, 0x01, 0xB8]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_439,
        relationship_type: RelationshipType::HasLowerPriorityThan,
    }],
};
