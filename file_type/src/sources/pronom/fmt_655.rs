use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_655: FileFormat = FileFormat {
    id: 1_454,
    puid: "fmt/655",
    name: "KryoFlux",
    extensions: &["raw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4B, 0x72, 0x79, 0x6F, 0x46, 0x6C, 0x75, 0x78, 0x20, 0x44, 0x69, 0x73, 0x6B,
                    0x53, 0x79, 0x73, 0x74, 0x65, 0x6D, 0x2C, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69,
                    0x6F, 0x6E, 0x3D, 0x32, 0x2E, 0x30, 0x30, 0x6E,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_455,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
