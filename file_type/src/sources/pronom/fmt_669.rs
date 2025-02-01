use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_669: FileFormat = FileFormat {
    id: 1_468,
    puid: "fmt/669",
    name: "Minolta RAW",
    extensions: &["mrw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0x4D, 0x52, 0x4D, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
