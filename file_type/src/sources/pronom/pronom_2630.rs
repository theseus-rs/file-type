use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_2630: FileFormat = FileFormat {
    id: 2_630,
    source_type: SourceType::Pronom,
    name: "Koala MicroIllustrator Graphic File",
    extensions: &["pic"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFF, 0x80, 0xC9, 0xC7, 0x1A])],
            },
        }],
    }],
    related_formats: &[],
};
