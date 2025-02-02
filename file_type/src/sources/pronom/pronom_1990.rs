use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1990: FileFormat = FileFormat {
    id: 1_990,
    source_type: SourceType::Pronom,
    name: "Cinema 4D",
    extensions: &["c4d"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(1),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x34, 0x44, 0x43, 0x34, 0x44, 0x36])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 1_327,
    }],
};
