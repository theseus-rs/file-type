use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_21: FileFormat = FileFormat {
    id: 694,
    puid: "fmt/21",
    name: "AutoCAD Drawing",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x4D, 0x43, 0x30, 0x2E, 0x30])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 710,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 695,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
    ],
};
