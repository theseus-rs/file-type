use crate::format::{
    ByteSequence, FileFormat, PositionType, Regex, RelatedFormat, RelationshipType, Signature,
    SourceType, Token,
};

pub(crate) const PRONOM_1446: FileFormat = FileFormat {
    id: 1_446,
    source_type: SourceType::Pronom,
    name: "Microsoft Expression Media",
    extensions: &["ivc"],
    media_types: &[],
    signatures: &[Signature {
        byte_sequences: &[
            ByteSequence {
                position_type: PositionType::BOF,
                offset: Some(4),
                regex: Regex {
                    tokens: &[Token::Literal(&[0x30, 0x32, 0x35, 0x69])],
                },
            },
            ByteSequence {
                position_type: PositionType::EOF,
                offset: Some(4),
                regex: Regex {
                    tokens: &[
                        Token::Literal(&[0x53, 0x56, 0x61, 0x72]),
                        Token::WildcardCountRange(8, 54),
                        Token::Literal(&[0x30, 0x32, 0x35, 0x69]),
                    ],
                },
            },
        ],
    }],
    related_formats: &[RelatedFormat {
        relationship_type: RelationshipType::IsPreviousVersionOf,
        id: 1_447,
    }],
};
