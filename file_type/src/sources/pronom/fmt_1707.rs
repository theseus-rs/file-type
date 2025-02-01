use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1707: FileFormat = FileFormat {
    id: 2_543,
    puid: "fmt/1707",
    name: "Persuasion Windows Document",
    extensions: &["pr3", "at3", "pn4", "at4"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
        id: 2_542,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
