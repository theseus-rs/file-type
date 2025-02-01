use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_95: FileFormat = FileFormat {
    id: 143,
    puid: "x-fmt/95",
    name: "Inkwriter/Notetaker Document",
    extensions: &["pwi"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x7B, 0x5C, 0x70, 0x77, 0x69])],
            },
        }],
    }],
    related_formats: &[],
};
