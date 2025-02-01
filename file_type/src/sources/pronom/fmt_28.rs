use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_28: FileFormat = FileFormat {
    id: 701,
    puid: "fmt/28",
    name: "AutoCAD Drawing",
    extensions: &["dwg"],
    media_types: &["image/vnd.dwg"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[0x41, 0x43, 0x31, 0x30, 0x30, 0x32])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 717,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 702,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 700,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
