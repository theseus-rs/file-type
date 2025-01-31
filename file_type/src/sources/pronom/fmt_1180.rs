use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1180: FileFormat = FileFormat {
    id: 1_990,
    puid: "fmt/1180",
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
        id: 1_327,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
