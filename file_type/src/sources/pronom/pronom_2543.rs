use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2543: FileFormat = FileFormat {
    id: 2_543,
    source_type: SourceType::Pronom,
    name: "Persuasion Windows Document",
    extensions: &["pr3", "at3", "pn4", "at4"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0x99]),
                    Token::WildcardCount(100),
                    Token::Literal(&[0x53, 0x50, 0x00, 0x03, 0x02]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 2_542,
    }],
};
