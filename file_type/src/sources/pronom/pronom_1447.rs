use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1447: FileFormat = FileFormat {
    id: 1_447,
    source_type: SourceType::Pronom,
    name: "Media View Pro",
    extensions: &["mpcatalog"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(4),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x30, 0x33, 0x30, 0x69])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(8),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x53, 0x56, 0x61, 0x72]),
                        Token::WildcardCountRange(8, 34),
                        Token::Literal(&[0x30, 0x33, 0x30, 0x69]),
                    ],
                },
            },
        ],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsSubsequentVersionOf,
        id: 1_446,
    }],
};
