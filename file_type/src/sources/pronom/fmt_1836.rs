use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1836: FileFormat = FileFormat {
    id: 2_688,
    puid: "fmt/1836",
    name: "Brio Query File",
    extensions: &["bqy"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x42, 0x52, 0x49, 0x46, 0x20, 0x42, 0x49, 0x4E,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
