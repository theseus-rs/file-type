use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_314: FileFormat = FileFormat {
    id: 1_059,
    puid: "fmt/314",
    name: "Play SID Audio",
    extensions: &["sid", "psid"],
    media_types: &["audio/prs.sid"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x50, 0x53, 0x49, 0x44, 0x00, 0x01, 0x00, 0x76,
                ])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_060,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
