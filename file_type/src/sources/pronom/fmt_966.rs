use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_966: FileFormat = FileFormat {
    id: 1_771,
    puid: "fmt/966",
    name: "AppleDouble Resource Fork",
    extensions: &[],
    media_types: &["multipart/appledouble"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x05, 0x16, 0x07, 0x00, 0x01, 0x00, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
