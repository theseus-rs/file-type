use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_2613: FileFormat = FileFormat {
    id: 2_613,
    source_type: SourceType::Pronom,
    name: "MacBinary",
    extensions: &["bin"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x00]),
                    Token::WildcardCount(73),
                    Token::Literal(&[0x00]),
                    Token::WildcardCount(7),
                    Token::Literal(&[0x00]),
                    Token::WildcardCount(19),
                    Token::Literal(&[0x6D, 0x42, 0x49, 0x4E]),
                    Token::WildcardCount(16),
                    Token::Literal(&[0x82]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 2_612,
    }],
};
