use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_35: FileFormat = FileFormat {
    id: 66,
    puid: "x-fmt/35",
    name: "Corel Presentation Exchange File",
    extensions: &["cmx"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x52, 0x49, 0x46]),
                    Token::Any(&[&[Token::Literal(&[0x46])], &[Token::Literal(&[0x58])]]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x43, 0x4D, 0x58, 0x31]),
                    Token::WildcardCount(62),
                    Token::Literal(&[0x32]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_741,
        relationship_type: RelationshipType::HasPriorityOver,
    }],
};
