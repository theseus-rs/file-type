use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1279: FileFormat = FileFormat {
    id: 2_097,
    puid: "fmt/1279",
    name: "Cindex Document",
    extensions: &["ucdx", "utpl"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x94, 0x70, 0x00, 0x00, 0x36,
                    0x01,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_096,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
