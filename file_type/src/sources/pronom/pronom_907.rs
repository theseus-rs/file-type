use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_907: FileFormat = FileFormat {
    id: 907,
    source_type: SourceType::Pronom,
    name: "PrimeOCR",
    extensions: &["pro"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x34, 0x30, 0x30, 0x2C]),
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
