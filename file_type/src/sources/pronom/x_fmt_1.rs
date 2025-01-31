use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_1: FileFormat = FileFormat {
    id: 8,
    puid: "x-fmt/1",
    name: "Microsoft Word for Macintosh Document",
    extensions: &["mcw"],
    media_types: &["application/msword"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0xFE, 0x34, 0x00]),
                    Token::Any(&[&[Token::Literal(&[0xC1])], &[Token::Literal(&[0x00])]]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 105,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 106,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_091,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
