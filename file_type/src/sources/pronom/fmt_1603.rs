use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1603: FileFormat = FileFormat {
    id: 2_430,
    puid: "fmt/1603",
    name: "TUNDRA",
    extensions: &["tnd"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x18, 0x54, 0x55, 0x4E, 0x44, 0x52, 0x41, 0x32, 0x34,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
