use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_845: FileFormat = FileFormat {
    id: 1_646,
    puid: "fmt/845",
    name: "ClarisWorks Drawing",
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
                    Token::Literal(&[0x00]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_536,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
