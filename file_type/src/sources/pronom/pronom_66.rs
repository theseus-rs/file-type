use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_66: FileFormat = FileFormat {
    id: 66,
    source_type: SourceType::Pronom,
    name: "Corel Presentation Exchange File",
    extensions: &["cmx"],
    media_types: &[],
    signatures: &[Signature {
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
        relationship_type: RelationshipType::HasPriorityOver,
        id: 2_741,
    }],
};
