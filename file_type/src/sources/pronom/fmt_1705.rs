use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1705: FileFormat = FileFormat {
    id: 2_541,
    puid: "fmt/1705",
    name: "Persuasion Mac Document",
    extensions: &["pn4"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x35]),
                    Token::WildcardCount(5),
                    Token::Literal(&[0x56, 0x88]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_540,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
