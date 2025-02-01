use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1715: FileFormat = FileFormat {
    id: 2_551,
    puid: "fmt/1715",
    name: "Applet Effect Factory Config File",
    extensions: &["data"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x43, 0x4F, 0x4E, 0x46, 0x49, 0x47, 0x20, 0x44, 0x41, 0x54, 0x41, 0x20, 0x46,
                    0x4F, 0x52, 0x20, 0x41, 0x50, 0x50, 0x4C, 0x45, 0x54, 0x20, 0x45, 0x46, 0x46,
                    0x45, 0x43, 0x54, 0x53, 0x20, 0x46, 0x41, 0x43, 0x54, 0x4F, 0x52, 0x59,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
