use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1816: FileFormat = FileFormat {
    id: 2_667,
    puid: "fmt/1816",
    name: "Adobe Swatch Exchange",
    extensions: &["ase"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x53, 0x45, 0x46, 0x00, 0x01])],
            },
        }],
    }],
    related_formats: &[],
};
