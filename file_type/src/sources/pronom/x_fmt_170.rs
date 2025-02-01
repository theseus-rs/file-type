use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_170: FileFormat = FileFormat {
    id: 242,
    puid: "x-fmt/170",
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
