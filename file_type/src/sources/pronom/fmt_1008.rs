use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1008: FileFormat = FileFormat {
    id: 1_813,
    puid: "fmt/1008",
    name: "DSS Pro",
    extensions: &["ds2"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x03, 0x64, 0x73, 0x32])],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 1_812,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
