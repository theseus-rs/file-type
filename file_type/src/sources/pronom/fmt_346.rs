use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_346: FileFormat = FileFormat {
    id: 1_091,
    puid: "fmt/346",
    name: "Microsoft Word for Macintosh Document",
    extensions: &["mcw"],
    media_types: &["application/msword"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0xFE, 0x32, 0x00, 0xC1])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 8,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 11,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 105,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 106,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
    ],
};
