use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1668: FileFormat = FileFormat {
    id: 2_504,
    puid: "fmt/1668",
    name: "Roxio Easy Media Creator Layout",
    extensions: &["roxio"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x00, 0xBB, 0x00, 0xBB, 0x01])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_505,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_506,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 2_502,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
