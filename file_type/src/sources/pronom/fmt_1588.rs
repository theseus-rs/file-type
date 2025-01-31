use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1588: FileFormat = FileFormat {
    id: 2_413,
    puid: "fmt/1588",
    name: "TGIF File Format",
    extensions: &["tgif", "obj"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x54, 0x47, 0x49, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
