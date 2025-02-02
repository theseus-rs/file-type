use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2541: FileFormat = FileFormat {
    id: 2_541,
    source_type: SourceType::Pronom,
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
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 2_540,
    }],
};
