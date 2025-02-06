use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2032: FileFormat = FileFormat {
    id: 2_032,
    source_type: SourceType::Pronom,
    name: "WordPerfect for Macintosh Document",
    extensions: &[],
    media_types: &["application/vnd.wordperfect"],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xFF, 0x57, 0x50, 0x43]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x01, 0x2C, 0x04]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
