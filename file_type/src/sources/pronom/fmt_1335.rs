use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1335: FileFormat = FileFormat {
    id: 2_153,
    puid: "fmt/1335",
    name: "Sony PictureGear Studio Binder",
    extensions: &["bxu", "bxt"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x38, 0x00, 0x00, 0x00, 0x54, 0x47, 0x42, 0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
