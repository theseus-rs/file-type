use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_266: FileFormat = FileFormat {
    id: 386,
    puid: "x-fmt/266",
    name: "GZIP Format",
    extensions: &["gz", "z"],
    media_types: &["application/gzip"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x1F, 0x8B, 0x08])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 2_008,
            relationship_type: RelationshipType::CanContain,
        },
        RelatedFormat {
            id: 2_009,
            relationship_type: RelationshipType::CanContain,
        },
        RelatedFormat {
            id: 2_029,
            relationship_type: RelationshipType::CanContain,
        },
        RelatedFormat {
            id: 2_286,
            relationship_type: RelationshipType::CanContain,
        },
    ],
};
