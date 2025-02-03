use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1448: FileFormat = FileFormat {
    id: 1_448,
    source_type: SourceType::Pronom,
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
        relationship_type: RelationshipType::HasLowerPriorityThan,
        id: 1_439,
    }],
};
