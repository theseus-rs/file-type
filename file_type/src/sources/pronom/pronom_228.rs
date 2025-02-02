use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_228: FileFormat = FileFormat {
    id: 228,
    source_type: SourceType::Pronom,
    name: "MacPaint Image",
    extensions: &["mac"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x00, 0x00, 0x00, 0x02, 0xFF, 0xFF, 0xFF, 0xFF,
                ])],
            },
        }],
    }],
    related_formats: &[],
};
