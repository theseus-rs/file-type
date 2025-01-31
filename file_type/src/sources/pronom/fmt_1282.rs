use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1282: FileFormat = FileFormat {
    id: 2_100,
    puid: "fmt/1282",
    name: "PFS:First Choice Document",
    extensions: &["doc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(9),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x47, 0x45, 0x52, 0x42, 0x49, 0x4C, 0x44, 0x4F, 0x43, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_101,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
