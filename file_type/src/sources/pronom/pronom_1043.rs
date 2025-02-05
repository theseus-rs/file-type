use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1043: FileFormat = FileFormat {
    id: 1_043,
    source_type: SourceType::Pronom,
    name: "Autodesk Animator (FlicLib)",
    extensions: &["fli"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x46, 0x4C, 0x49, 0x42, 0x02, 0x00]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x00]),
                    Token::WildcardCount(3),
                    Token::Literal(&[0x00, 0x00]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x00]),
                    Token::WildcardCount(5),
                    Token::Literal(&[0x00, 0x00]),
                    Token::WildcardCount(2),
                    Token::Literal(&[0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
