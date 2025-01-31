use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1546: FileFormat = FileFormat {
    id: 2_371,
    puid: "fmt/1546",
    name: "Daisy-Dot Font File",
    extensions: &["nlq"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x41, 0x49, 0x53, 0x59, 0x2D, 0x44, 0x4F, 0x54, 0x20, 0x4E, 0x4C, 0x51,
                    0x20, 0x46, 0x4F, 0x4E, 0x54,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
