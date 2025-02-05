use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_465: FileFormat = FileFormat {
    id: 465,
    source_type: SourceType::Pronom,
    name: "Paradox Database Memo Field (Binary Large Object)",
    extensions: &["dbq", "mb"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00, 0x01, 0x00]),
                    Token::WildcardCount(8),
                    Token::Literal(&[0x00, 0x10, 0x00, 0x10]),
                    Token::WildcardCount(1),
                    Token::Literal(&[0x10, 0x40, 0x00, 0x00, 0x08]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
