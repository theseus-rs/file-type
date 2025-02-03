use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1605: FileFormat = FileFormat {
    id: 1_605,
    source_type: SourceType::Pronom,
    name: "XAML Binary Format",
    extensions: &["xbf"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x58, 0x42, 0x46, 0x00])],
            },
        }],
    }],
    related_formats: &[],
};
