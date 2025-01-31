use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_37: FileFormat = FileFormat {
    id: 733,
    puid: "fmt/37",
    name: "Microsoft Word for Windows Document",
    extensions: &["doc"],
    media_types: &["application/msword"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x9B, 0xA5]),
                    Token::WildcardCount(16),
                    Token::Literal(&[0x00, 0x00, 0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 734,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
