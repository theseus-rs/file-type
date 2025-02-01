use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1426: FileFormat = FileFormat {
    id: 2_244,
    puid: "fmt/1426",
    name: "MacDraw",
    extensions: &[],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x44, 0x52, 0x57, 0x47, 0x4D, 0x44, 0x00, 0x06,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
