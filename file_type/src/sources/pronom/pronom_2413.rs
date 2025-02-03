use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2413: FileFormat = FileFormat {
    id: 2_413,
    source_type: SourceType::Pronom,
    name: "TGIF File Format",
    extensions: &["tgif", "obj"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x25, 0x54, 0x47, 0x49, 0x46])],
            },
        }],
    }],
    related_formats: &[],
};
