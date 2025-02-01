use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1651: FileFormat = FileFormat {
    id: 2_478,
    puid: "fmt/1651",
    name: "Garmin Flexible and Interoperable Data Transfer File",
    extensions: &["fit"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(9),
            regex: Regex {
                tokens: &[Token::Literal(&[0x46, 0x49, 0x54])],
            },
        }],
    }],
    related_formats: &[],
};
