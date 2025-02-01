use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_433: FileFormat = FileFormat {
    id: 827,
    puid: "x-fmt/433",
    name: "3DM",
    extensions: &["3dm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x33, 0x44, 0x20, 0x47, 0x65, 0x6F, 0x6D, 0x65, 0x74, 0x72, 0x79, 0x20, 0x46,
                    0x69, 0x6C, 0x65, 0x20, 0x46, 0x6F, 0x72, 0x6D, 0x61, 0x74, 0x20, 0x20, 0x20,
                    0x20, 0x20, 0x20, 0x20, 0x20, 0x31, 0x01, 0x00, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 828,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
