use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_1482: FileFormat = FileFormat {
    id: 1_482,
    source_type: SourceType::Pronom,
    name: "Advanced Function Presentation",
    extensions: &["afp"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x5A]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0xD3, 0xA8]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
