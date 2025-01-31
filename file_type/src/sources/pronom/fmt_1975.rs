use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1975: FileFormat = FileFormat {
    id: 2_842,
    puid: "fmt/1975",
    name: "ICC Profile",
    extensions: &["icc", "icm"],
    media_types: &["application/vnd.iccprofile"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(8),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x02]),
                    Token::WildcardCount(27),
                    Token::Literal(&[0x61, 0x63, 0x73, 0x70]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 2_843,
        relationship_type: RelationshipType::IsPreviousVersionOf,
    }],
};
