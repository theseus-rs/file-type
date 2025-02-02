use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1216: FileFormat = FileFormat {
    id: 1_216,
    source_type: SourceType::Pronom,
    name: "CorelDraw Drawing",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x50, 0x4B]),
                    Token::WildcardCount(48),
                    Token::Literal(&[0x52, 0x49, 0x46]),
                    Token::Any(&[&[Token::Literal(&[0x46])], &[Token::Literal(&[0x58])]]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x43, 0x44, 0x52, 0x46]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 382,
    }],
};
