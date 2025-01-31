use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1046: FileFormat = FileFormat {
    id: 1_851,
    puid: "fmt/1046",
    name: "Draco File Format",
    extensions: &["drc"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x44, 0x52, 0x41, 0x43, 0x4F, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
