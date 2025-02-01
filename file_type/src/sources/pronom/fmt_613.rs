use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_613: FileFormat = FileFormat {
    id: 1_409,
    puid: "fmt/613",
    name: "RAR Archive",
    extensions: &["rar"],
    media_types: &["application/vnd.rar"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x52, 0x61, 0x72, 0x21, 0x1A, 0x07, 0x01, 0x00,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
