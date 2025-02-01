use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_868: FileFormat = FileFormat {
    id: 1_672,
    puid: "fmt/868",
    name: "MySQL Table Definition Format",
    extensions: &["frm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xFE, 0x01]),
                    Token::WildcardCount(3),
                    Token::Literal(&[0x00]),
                    Token::WildcardCount(21),
                    Token::Literal(&[0x02]),
                    Token::WildcardCount(13),
                    Token::Literal(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 533,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
