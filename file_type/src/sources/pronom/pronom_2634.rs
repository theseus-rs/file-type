use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2634: FileFormat = FileFormat {
    id: 2_634,
    source_type: SourceType::Pronom,
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
