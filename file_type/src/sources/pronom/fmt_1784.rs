use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1784: FileFormat = FileFormat {
    id: 2_634,
    puid: "fmt/1784",
    name: "Animatic Film Format",
    extensions: &["flm"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(48),
            regex: Regex {
                tokens: &[Token::Literal(&[0x27, 0x18, 0x28, 0x18])],
            },
        }],
    }],
    related_formats: &[],
};
