use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_648: FileFormat = FileFormat {
    id: 1_447,
    puid: "fmt/648",
    name: "Media View Pro",
    extensions: &["mpcatalog"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
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
        id: 1_446,
        relationship_type: RelationshipType::IsSubsequentVersionOf,
    }],
};
