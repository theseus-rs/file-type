use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1183: FileFormat = FileFormat {
    id: 1_993,
    puid: "fmt/1183",
    name: "MicroStation Material Palette",
    extensions: &["pal"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x4D, 0x53, 0x5F, 0x50, 0x4C, 0x2D, 0x2D, 0x30,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
