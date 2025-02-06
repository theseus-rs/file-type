use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1960: FileFormat = FileFormat {
    id: 1_960,
    source_type: SourceType::Pronom,
    name: "4X Movie File",
    extensions: &["4xm", "4xa"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x52, 0x49, 0x46, 0x46]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x34, 0x58, 0x4D, 0x56, 0x4C, 0x49, 0x53, 0x54]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x48, 0x45, 0x41, 0x44]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::HasPriorityOver,
        id: 2_741,
    }],
};
