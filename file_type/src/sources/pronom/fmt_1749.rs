use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1749: FileFormat = FileFormat {
    id: 2_595,
    puid: "fmt/1749",
    name: "Canon MIF File",
    extensions: &["mif"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x49, 0x01, 0x01, 0x02])],
            },
        }],
    }],
    related_formats: &[],
};
