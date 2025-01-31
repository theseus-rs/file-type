use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, RelatedFormat,
    RelationshipType, Token,
};

pub(crate) const FMT_1670: FileFormat = FileFormat {
    id: 2_506,
    puid: "fmt/1670",
    name: "Roxio Audio Project File",
    extensions: &["rox"],
    media_types: &[],
    internal_signatures: &[InternalSignature {
        byte_sequences: &[ByteSequence {
            position_type: PositionType::BOF,
            offset: Some(160),
            regex: Regex {
                tokens: &[Token::Literal(&[
                    0x31, 0x30, 0x32, 0x35, 0x33, 0x43, 0x34, 0x43, 0x2D, 0x32, 0x32, 0x39, 0x44,
                    0x2D, 0x34, 0x63, 0x38, 0x37, 0x2D, 0x38, 0x44, 0x31, 0x44, 0x2D, 0x31, 0x36,
                    0x39, 0x45, 0x46, 0x44, 0x46, 0x45, 0x44, 0x38, 0x36, 0x39,
                ])],
            },
        }],
    }],
    related_formats: &[
        RelatedFormat {
            id: 638,
            relationship_type: RelationshipType::HasPriorityOver,
        },
        RelatedFormat {
            id: 2_503,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
        RelatedFormat {
            id: 2_504,
            relationship_type: RelationshipType::IsSubsequentVersionOf,
        },
    ],
};
