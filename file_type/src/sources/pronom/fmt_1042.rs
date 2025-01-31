use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1042: FileFormat = FileFormat {
    id: 1_847,
    puid: "fmt/1042",
    name: "WordPerfect Graphics Metafile",
    extensions: &["wpg"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xFF, 0x57, 0x50, 0x43]),
                    Token::WildcardCount(4),
                    Token::Literal(&[0x01, 0x16, 0x02, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[RelatedFormat {
        id: 738,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
