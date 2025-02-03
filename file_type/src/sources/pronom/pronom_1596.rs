use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1596: FileFormat = FileFormat {
    id: 1_596,
    source_type: SourceType::Pronom,
    name: "Apple ProRes",
    extensions: &["mov"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(4),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x66, 0x72, 0x65, 0x65]),
                    Token::WildcardCount(12),
                    Token::Literal(&[0x69, 0x63, 0x70, 0x66]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 658,
    }],
};
