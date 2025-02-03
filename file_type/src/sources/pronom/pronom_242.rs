use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_242: FileFormat = FileFormat {
    id: 242,
    source_type: SourceType::Pronom,
    name: "PC Paint Bitmap",
    extensions: &["pic"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x34, 0x12]),
                    Token::WildcardCount(9),
                    Token::Literal(&[0xFF]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
