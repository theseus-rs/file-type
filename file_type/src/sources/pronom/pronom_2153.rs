use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2153: FileFormat = FileFormat {
    id: 2_153,
    source_type: SourceType::Pronom,
    name: "Sony PictureGear Studio Binder",
    extensions: &["bxu", "bxt"],
    media_types: &[],
    signatures: &[Signature {
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
