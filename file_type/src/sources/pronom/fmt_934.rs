use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_934: FileFormat = FileFormat {
    id: 1_739,
    puid: "fmt/934",
    name: "Simple Vector Format",
    extensions: &["svf"],
    media_types: &["image/vnd-svf"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x56, 0x46, 0x20, 0x76, 0x32])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_738,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
