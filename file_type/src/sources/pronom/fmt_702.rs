use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_702: FileFormat = FileFormat {
    id: 1_501,
    puid: "fmt/702",
    name: "Universal 3D File Format",
    extensions: &["u3d"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x55, 0x33, 0x44, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
