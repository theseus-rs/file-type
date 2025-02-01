use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_465: FileFormat = FileFormat {
    id: 1_252,
    puid: "fmt/465",
    name: "CorelDraw Drawing",
    extensions: &["cdr"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x52, 0x49, 0x46, 0x46]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x43, 0x44, 0x52, 0x34, 0x76, 0x72, 0x73, 0x6E]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_741,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
