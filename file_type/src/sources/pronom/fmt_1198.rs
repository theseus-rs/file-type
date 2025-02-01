use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1198: FileFormat = FileFormat {
    id: 2_008,
    puid: "fmt/1198",
    name: "RData",
    extensions: &["rdata"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x52, 0x44, 0x58, 0x32, 0x0A, 0x58, 0x0A])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 386,
        relationship_type: RelationshipType::CanBeContainedBy,
    }],
};
