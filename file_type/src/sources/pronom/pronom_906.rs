use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, SourceType, Token,
};

pub(crate) const PRONOM_906: FileFormat = FileFormat {
    id: 906,
    source_type: SourceType::Pronom,
    name: "Prime OCR",
    extensions: &["pro"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x33, 0x39, 0x30, 0x2C]),
                    Token::SingleWildcard,
                    Token::Literal(&[0x2C]),
                    Token::WildcardCount(3),
                    Token::Literal(&[0x2C]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
