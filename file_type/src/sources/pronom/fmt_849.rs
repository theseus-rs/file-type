use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_849: FileFormat = FileFormat {
    id: 1_650,
    puid: "fmt/849",
    name: "ClarisWorks Painting",
    extensions: &["cwk"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x02]),
                    Token::WildcardCount(3),
                    Token::Literal(&[0x42, 0x4F, 0x42, 0x4F]),
                    Token::WildcardCount(240),
                    Token::Literal(&[0x04]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_536,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
