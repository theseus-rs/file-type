use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_846: FileFormat = FileFormat {
    id: 1_647,
    puid: "fmt/846",
    name: "ClarisWorks Word Processor",
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
                    Token::Literal(&[0x01]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_536,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
