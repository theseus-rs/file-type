use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_94: FileFormat = FileFormat {
    id: 662,
    puid: "fmt/94",
    name: "Virtual Reality Modeling Language",
    extensions: &["wrl"],
    media_types: &["model/vrml"],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(0),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x23, 0x56, 0x52, 0x4D, 0x4C, 0x20, 0x56, 0x32, 0x2E, 0x30, 0x20, 0x75, 0x74,
                    0x66, 0x38,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 1_367,
            relationship_type: RelationshipType::IsPreviousVersionOf,
        },
        RelatedFormat {
            id: 661,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
