use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1953: FileFormat = FileFormat {
    id: 2_817,
    puid: "fmt/1953",
    name: "Zoom Project Settings",
    extensions: &["hprj"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5A, 0x4F, 0x4F, 0x4D, 0x20, 0x48, 0x35, 0x20, 0x70, 0x72, 0x6A, 0x65, 0x63,
                    0x74, 0x66, 0x69, 0x6C, 0x65,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_819,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
