use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_665: FileFormat = FileFormat {
    id: 1_464,
    puid: "fmt/665",
    name: "Chasys Draw image file",
    extensions: &["cd5"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x5F, 0x43, 0x44, 0x35, 0x10, 0x00, 0x00, 0x00, 0x1A, 0x00, 0x03,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
