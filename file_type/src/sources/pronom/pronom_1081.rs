use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1081: FileFormat = FileFormat {
    id: 1_081,
    source_type: SourceType::Pronom,
    name: "Graphic Workshop for Windows Thumbnail File",
    extensions: &["thn"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x54, 0x48, 0x4E, 0x4C])],
            },
        }],
    }],
    related_formats: &[],
};
