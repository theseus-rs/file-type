use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1481: FileFormat = FileFormat {
    id: 2_304,
    puid: "fmt/1481",
    name: "Micrografx In-A-Vision Drawing",
    extensions: &["pic"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x01, 0xFF, 0x01, 0x04, 0x03, 0x06, 0x00, 0x01, 0x00, 0x04, 0x01, 0xFF, 0xFF,
                    0xFF, 0x00, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
