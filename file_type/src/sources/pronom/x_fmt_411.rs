use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const X_FMT_411: FileFormat = FileFormat {
    id: 776,
    puid: "x-fmt/411",
    name: "Windows Portable Executable",
    extensions: &["exe", "dll", "sys"],
    media_types: &["application/vnd.microsoft.portable-executable"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[
                    Token::Literal(&[0x4D, 0x5A]),
                    Token::AnyWildcard,
                    Token::Literal(&[0x50, 0x45, 0x00, 0x00]),
                ],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_704,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 1_705,
            relationship_type: RelationshipType::HasLowerPriorityThan,
        },
        RelatedFormat {
            id: 774,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 775,
            relationship_type: RelationshipType::HasPriorityOver,
        },
    ],
};
