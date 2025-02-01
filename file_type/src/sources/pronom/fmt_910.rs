use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_910: FileFormat = FileFormat {
    id: 1_715,
    puid: "fmt/910",
    name: "CRAM File Format",
    extensions: &["cram"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x43, 0x52, 0x41, 0x4D, 0x02])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_716,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 1_714,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
