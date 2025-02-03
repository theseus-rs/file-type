use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1411: FileFormat = FileFormat {
    id: 1_411,
    source_type: SourceType::Pronom,
    name: "Gimp Image File Format",
    extensions: &["xcf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x67, 0x69, 0x6D, 0x70, 0x20, 0x78, 0x63, 0x66, 0x20,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
