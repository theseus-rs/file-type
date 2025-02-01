use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_318: FileFormat = FileFormat {
    id: 1_063,
    puid: "fmt/318",
    name: "Secure DjVU",
    extensions: &["djvu", "djv"],
    media_types: &["image/vnd.djvu", "image/x-djvu"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x53, 0x44, 0x4A, 0x56])],
            },
        }],
    }],
    related_formats: &[],
};
