use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1837: FileFormat = FileFormat {
    id: 2_689,
    puid: "fmt/1837",
    name: "WordPerfect Presentations",
    extensions: &["shw"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xFF, 0x57, 0x50, 0x43]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x0F, 0x0A, 0x02, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[],
};
